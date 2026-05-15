; Inject Jinja2 syntax into dbt SQL files.
;
; dbt models embed Jinja2 template syntax inside SQL:
;   {{ ref('model_name') }}         — expression blocks
;   {% if condition %}...{% endif %} — statement blocks
;   {# comment #}                   — comment blocks
;
; The sql grammar treats these as opaque tokens. This file injects the
; jinja2 language into those regions so Zed highlights them correctly.
;
; NOTE: Full Jinja injection requires a grammar that surfaces Jinja
; nodes as distinct AST nodes (e.g. tree-sitter-jinja2). Until a
; combined dbt grammar is added, this file captures string-embedded
; Jinja via regex patterns.

; Jinja expression:  {{ ... }}
((literal) @injection.content
  (#match? @injection.content "\\{\\{.*\\}\\}")
  (#set! injection.language "jinja2")
  (#set! injection.combined))

; Jinja statement:  {% ... %}
((literal) @injection.content
  (#match? @injection.content "\\{%.*%\\}")
  (#set! injection.language "jinja2")
  (#set! injection.combined))

; Jinja comment:  {# ... #}
((comment) @injection.content
  (#match? @injection.content "\\{#.*#\\}")
  (#set! injection.language "jinja2")
  (#set! injection.combined))
