# Example dbt Project

This sample project exists to test the `dbt` Zed extension in a realistic dbt workspace.

It includes:
- Jinja templating in SQL models
- `ref()` usage across models
- source configuration in YAML
- column descriptions and tests in schema files

## Quick start

1. Create a local profile for this project:

```bash
cp profiles.yml.example ~/.dbt/profiles.yml
```

2. Update credentials in `~/.dbt/profiles.yml` for your warehouse.

3. From this directory, run:

```bash
dbt parse
dbt run
```

4. Open `example_dbt/` in Zed and test:
- syntax highlighting
- SQL formatting
- Jinja formatting
- language server startup

The sample uses Snowflake-style configuration because the parent project is aimed at Snowflake dbt workflows.
