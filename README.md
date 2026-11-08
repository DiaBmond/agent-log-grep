# agent-log-grep

A lightweight Rust CLI tool for reading and searching AI agent logs, inspired by the MiniGrep project from *The Rust Programming Language* book.

The project aims to evolve into a provider-agnostic tool for exploring and analyzing AI agent activity.

## Features

- **Search** text in log files.
- **Show** log contents with line numbers.
- **Case-insensitive search** using `-i`, `--ignore-case`, or the `IGNORE_CASE` environment variable.
- **Line-based log reading** using Rust's `BufReader`.
- **Provider-agnostic raw logs** — no AI-specific log schema required.
- **No external dependencies** — built with the Rust standard library.

## Getting Started

### Prerequisites

- Rust and Cargo installed.

### Clone and Build

```bash
git clone <repository-url>
cd agent-log-grep
cargo build
```

Replace `<repository-url>` with the actual repository URL.

## Usage

### Search a log file

```bash
cargo run -- search "token_count" session.jsonl
```

Search results include their original line numbers:

```text
12: {"type":"event_msg","payload":{"type":"token_count"}}
```

### Case-insensitive search

Use `-i` or `--ignore-case` before the query:

```bash
cargo run -- search -i "error" session.jsonl
cargo run -- search --ignore-case "error" session.jsonl
```

Alternatively, use the `IGNORE_CASE` environment variable:

```bash
IGNORE_CASE=1 cargo run -- search "error" session.jsonl
```

### Show a log file

```bash
cargo run -- show session.jsonl
```

This prints every line of the file with its line number.

### Search Codex session logs

Codex session logs may be stored in:

```text
~/.codex/sessions/YYYY/MM/DD/rollout-*.jsonl
```

You can pass an individual session file to `search` or `show`.

The current version treats JSONL records as raw text. It does not parse or interpret JSON fields.

## Project Structure

```text
src/
├── main.rs       # Application entry point
├── lib.rs        # Command execution
├── cli.rs        # CLI argument parsing
├── log.rs        # Log reading and LogRecord
└── search.rs     # Text searching

tests/
└── integration.rs # End-to-end CLI integration tests
```

## Testing

Run all tests:

```bash
cargo test
```

Run only integration tests:

```bash
cargo test --test integration
```

The project includes unit tests for CLI parsing, log reading, and search behavior, as well as integration tests that execute the CLI binary.

## Current Limitations

- Only individual log files are supported.
- Log contents are treated as raw UTF-8 text.
- JSON fields are not parsed or interpreted.
- Log analysis and reporting are not yet implemented.

## Roadmap

- [x] CLI commands: `search` and `show`
- [x] Case-sensitive and case-insensitive search
- [x] Line-numbered log records
- [x] Unit and integration tests
- [ ] Recursive directory search
- [ ] Log filtering
- [ ] Provider-specific log parsers
- [ ] AI agent activity and tool-call analysis
- [ ] Token usage analysis

## License

MIT License.
