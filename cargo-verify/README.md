# cargo-verify

Datalog-gated promotion system for Rust. Extract facts from docstrings and gate CI promotion on declared-vs-actual compliance.

## Installation

```bash
cd ~/repos/portfolio/cargo-verify
cargo install --path .
```

## Usage

```bash
# Verify current project
cargo verify

# Verify specific manifest
cargo verify --manifest-path /path/to/Cargo.toml

# JSON output for CI
cargo verify --format json

# Skip coverage (if llvm-cov not installed)
cargo verify --no-coverage

# Quiet mode (errors only)
cargo verify -q

# Use cached test results
cargo verify --check
```

## Docstring Format

Add Datalog facts to your function docstrings:

```rust
/// Adds two numbers
///
/// ```datalog
/// test("add", pass).
/// coverage("add", 100.0).
/// complexity("add", 1).
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Supported Facts

- `test("name", pass)` or `test("name", fail)` - Expected test result
- `coverage("name", percentage)` - Minimum coverage requirement
- `complexity("name", score)` - Maximum cyclomatic complexity

## CLI Options

| Option | Description |
|--------|-------------|
| `--manifest-path` | Path to Cargo.toml |
| `--check` | Skip tests, use cached results |
| `--format` | Output: `human` or `json` |
| `--no-coverage` | Skip coverage verification |
| `--no-complexity` | Skip complexity verification |
| `-q` | Quiet mode (errors only) |

## Exit Codes

- `0` - All declarations verified, no violations
- `1` - Violations found (CI failure)

## CI Integration

Add to your `.github/workflows/ci.yml`:

```yaml
verify:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-action@stable
    - run: cargo install cargo-llvm-cov
    - run: cargo verify --format json
```

## How It Works

1. **Extract**: Parses `/// ```datalog` blocks from all `.rs` files using `syn`
2. **Collect**: Runs `cargo test --message-format=json` and `cargo llvm-cov`
3. **Verify**: Loads declared and actual facts into Ascent Datalog engine
4. **Report**: Compares and reports violations with file:line locations

## Example Violations

```
================================================================================
cargo-verify
================================================================================
✗ 2/3 tests verified, FAILED
--------------------------------------------------------------------------------
- VIOLATIONS -
  → div
     src/lib.rs line 12
     ERROR: coverage is 87.5%, below declared 95.0%
--------------------------------------------------------------------------------
- WARNINGS -
  ⚠ div_by_zero: No coverage data for 'div_by_zero'
```

## Architecture

```
main.rs      - CLI entry point, orchestration
extract.rs   - Parse docstrings with syn
parse.rs     - Datalog parser for declarations
collect.rs   - Run cargo test/llvm-cov
engine.rs    - Ascent Datalog verification
report.rs    - Human/JSON output formatting
```

## Development

```bash
# Run tests
cargo test --all

# Build
cargo build --release

# Install locally
cargo install --path .

# Verify example project
cd examples
cargo verify
```

## License

MIT OR Apache-2.0
