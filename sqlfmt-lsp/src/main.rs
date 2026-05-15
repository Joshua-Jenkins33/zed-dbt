//! sqlfmt-lsp: a minimal LSP server that wraps sqlfmt.
//!
//! It implements just enough of the Language Server Protocol for Zed to
//! invoke sqlfmt when the user saves a dbt SQL file:
//!
//!   - initialize / initialized
//!   - textDocument/didOpen, didChange, didClose
//!   - textDocument/formatting  ← runs `sqlfmt -` on the document content
//!   - shutdown / exit
//!
//! Install sqlfmt before using this server:
//!   pip install shandy-sqlfmt[jinjafmt]

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
};

use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// LSP server state
// ---------------------------------------------------------------------------

struct Server {
    documents: HashMap<String, String>,
}

impl Server {
    fn new() -> Self {
        Server {
            documents: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: Value) -> Option<Value> {
        let id = msg.get("id").cloned();
        let method = msg["method"].as_str().unwrap_or("");

        match method {
            "initialize" => respond(
                id,
                json!({
                    "capabilities": {
                        "textDocumentSync": {
                            "openClose": true,
                            // 1 = full document sync on every change
                            "change": 1
                        },
                        "documentFormattingProvider": true
                    },
                    "serverInfo": {
                        "name": "sqlfmt-lsp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            ),

            "initialized" | "$/cancelRequest" => None,

            "textDocument/didOpen" => {
                if let (Some(uri), Some(text)) = (
                    msg["params"]["textDocument"]["uri"].as_str(),
                    msg["params"]["textDocument"]["text"].as_str(),
                ) {
                    self.documents.insert(uri.to_string(), text.to_string());
                }
                None
            }

            "textDocument/didChange" => {
                if let Some(uri) = msg["params"]["textDocument"]["uri"].as_str() {
                    if let Some(changes) = msg["params"]["contentChanges"].as_array() {
                        if let Some(text) = changes.last().and_then(|c| c["text"].as_str()) {
                            self.documents.insert(uri.to_string(), text.to_string());
                        }
                    }
                }
                None
            }

            "textDocument/didClose" => {
                if let Some(uri) = msg["params"]["textDocument"]["uri"].as_str() {
                    self.documents.remove(uri);
                }
                None
            }

            "textDocument/formatting" => {
                let id = id?;
                let uri = msg["params"]["textDocument"]["uri"].as_str()?;
                let content = self.documents.get(uri)?.clone();

                match run_sqlfmt(&content) {
                    Ok(formatted) if formatted != content => {
                        respond(Some(id), json!(full_replace_edit(&content, &formatted)))
                    }
                    Ok(_) => respond(Some(id), json!([])),
                    Err(e) => {
                        eprintln!("sqlfmt-lsp: formatting error: {e}");
                        respond(Some(id), json!([]))
                    }
                }
            }

            "shutdown" => respond(id, json!(null)),

            // "exit" is handled in the main loop; nothing to respond here.
            "exit" => None,

            _ => {
                if let Some(id) = id {
                    Some(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32601, "message": "method not found" }
                    }))
                } else {
                    None
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// sqlfmt invocation
// ---------------------------------------------------------------------------

/// Pipe `content` through `sqlfmt -` and return the formatted output.
/// sqlfmt reads SQL (with optional Jinja) from stdin and writes to stdout.
fn run_sqlfmt(content: &str) -> io::Result<String> {
    let mut child = Command::new("sqlfmt")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .expect("stdin was piped")
        .write_all(content.as_bytes())?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("sqlfmt exited {}: {stderr}", output.status),
        ))
    }
}

// ---------------------------------------------------------------------------
// LSP text edit helpers
// ---------------------------------------------------------------------------

/// Build a single LSP TextEdit that replaces the entire document.
fn full_replace_edit(original: &str, formatted: &str) -> Vec<Value> {
    // Split on '\n' so trailing newlines produce a final empty element,
    // giving the correct last-line + last-character position.
    let lines: Vec<&str> = original.split('\n').collect();
    let end_line = lines.len().saturating_sub(1);
    let end_char = lines.last().map(|l| l.len()).unwrap_or(0);

    vec![json!({
        "range": {
            "start": { "line": 0, "character": 0 },
            "end":   { "line": end_line, "character": end_char }
        },
        "newText": formatted
    })]
}

// ---------------------------------------------------------------------------
// LSP transport (Content-Length framing over stdin/stdout)
// ---------------------------------------------------------------------------

fn respond(id: Option<Value>, result: Value) -> Option<Value> {
    id.map(|id| json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

fn read_msg(reader: &mut impl BufRead) -> io::Result<Option<Value>> {
    let mut content_length: usize = 0;

    // Read headers until blank line.
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Ok(None); // EOF
        }
        let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');
        if trimmed.is_empty() {
            break;
        }
        if let Some(val) = trimmed.strip_prefix("Content-Length: ") {
            content_length = val.parse().unwrap_or(0);
        }
    }

    if content_length == 0 {
        return Ok(None);
    }

    let mut buf = vec![0u8; content_length];
    reader.read_exact(&mut buf)?;

    serde_json::from_slice(&buf)
        .map(Some)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_msg(writer: &mut impl Write, msg: &Value) -> io::Result<()> {
    let body = serde_json::to_string(msg).expect("serialization is infallible");
    write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
    writer.flush()
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut server = Server::new();

    loop {
        match read_msg(&mut reader) {
            Ok(Some(msg)) => {
                let is_exit = msg["method"].as_str() == Some("exit");
                if let Some(response) = server.handle(msg) {
                    write_msg(&mut writer, &response)?;
                }
                if is_exit {
                    break;
                }
            }
            Ok(None) => break, // EOF — client closed the pipe
            Err(e) => {
                eprintln!("sqlfmt-lsp: read error: {e}");
                break;
            }
        }
    }

    Ok(())
}
