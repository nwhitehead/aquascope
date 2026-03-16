# Aquascope CLI

## Testing

Run testing as normal:

```bash
cargo test
```

The test mechanism for `aquascope_cli` is to check output against
various golden files stored in `crates/aquascope_cli/crates/aquascope_cli/src/main/testdata/`.
To regenerate the golden files use:

```bash
GOLDIE_UPDATE=1 cargo test
```
