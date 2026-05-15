# zed-dbt

Configuration and workflows for dbt and SQL development in the Zed IDE for Snowflake, including formatting and type hinting. The name should be `dbt`.

## Project Overview

This project configures a development environment for dbt and SQL in Zed, with:
- SQL formatting via `sqlfmt`
- dbt model type hinting and linting
- syntax highlighting for SQL files with jinja in them
- graying out the text of files that have compiled code in them, to help prevent you from editing compiled SQL (as opposed to your actual model)
- Zed IDE language server and formatter integration

## Todo List

- [x] Decide on a language to use for the Zed plugin — **Rust (compiled to WebAssembly)**. Zed extensions must be written in Rust using the `zed_extension_api` crate; there is no alternative.
- [x] Build out a repository structure for the project
- [x] Build out a basic plugin that allows you toggle running `sqlfmt` on save

## Semantic Versioning

I want to use semantic versioning for managing iterations, starting at version 0.1.0.

- Increment hotfix versions (z) when fixing bugs x.y.z+1.
- Increment minor versions (y) when adding new features: x.y+1.z
- Increment major versions (x) when introducing breaking, non-backwards compatible changes: x+1.y.z

Manage versions with git tags.

## Git Workflow (Trunk-Based Development)

This project uses trunk-based development. `main` is the trunk — it is always releasable.

### Starting a Feature Branch

```bash
# Ensure main is up to date before branching
git checkout main
git pull origin main

# Create a short-lived feature branch
git checkout -b feature/<short-description>
# e.g., git checkout -b feature/add-sqlfmt-config
```

Branch naming: `feature/<kebab-case-description>` (e.g., `feature/dbt-profiles-setup`)

### Keeping Your Branch Current

Rebase onto main frequently to avoid drift:

```bash
git fetch origin
git rebase origin/main
```

### Merging Back to Main

Feature branches should be short-lived (hours to a couple of days at most).

```bash
# Rebase onto latest main before merging
git fetch origin
git rebase origin/main

# Switch to main and merge (fast-forward preferred)
git checkout main
git pull origin main
git merge --ff-only feature/<short-description>
git push origin main

# Delete the branch after merging
git branch -d feature/<short-description>
git push origin --delete feature/<short-description>
```

If a fast-forward merge isn't possible, prefer squashing to keep main history clean:

```bash
git merge --squash feature/<short-description>
git commit -m "<concise description of the change>"
```

### Commit Style

- Use the imperative mood: `Add sqlfmt config`, not `Added sqlfmt config`
- Keep the subject line under 72 characters
- Reference issues or context in the body when non-obvious

## dbt Conventions

- Models live in `models/` and follow the `<layer>_<entity>` naming convention (e.g., `stg_orders`, `fct_revenue`)
- Always define column-level `description` and `tests` in accompanying `.yml` schema files
- Use `ref()` for all cross-model references; never hardcode schema or table names
- Staging models should be 1:1 with source tables; transformation logic belongs in intermediate or mart layers

## SQL Style

- Keywords in **uppercase** (`SELECT`, `FROM`, `WHERE`, `JOIN`)
- One column per line in `SELECT` clauses
- Trailing commas on all but the last column
- CTEs preferred over subqueries; each CTE on its own named block
- Formatting enforced by `sqlfmt` — see Zed config below

## Zed IDE Configuration

### Installing the extension

Load this extension locally in Zed via **Extensions → Install Dev Extension** and point it at this repository root.

### Installing dependencies

```bash
# 1. Install sqlfmt with Jinja support for dbt
pip install shandy-sqlfmt[jinjafmt]

# 2. Build and install the LSP wrapper from this repo
cargo install --path sqlfmt-lsp
```

`sqlfmt-lsp` (in `sqlfmt-lsp/`) is a thin stdin/stdout LSP server that bridges Zed's `textDocument/formatting` requests to sqlfmt.

### Toggle sqlfmt on save

Add to `~/.config/zed/settings.json`:

```json
{
  "languages": {
    "dbt": {
      "format_on_save": "on"
    }
  }
}
```

Set to `"off"` to disable formatting without uninstalling the extension.

### Override the sqlfmt-lsp binary path

If `sqlfmt-lsp` is not on your `PATH`:

```json
{
  "lsp": {
    "sqlfmt": {
      "binary": {
        "path": "/path/to/sqlfmt-lsp"
      }
    }
  }
}
```
