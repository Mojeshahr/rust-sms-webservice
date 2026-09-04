# Agent guide

Runnable Rust examples for the Payam Resan SMS web service. One file per API
method, and every file has to work on its own.

## Rule one: exactly one dependency

Rust has neither an HTTP client nor a JSON parser in its standard library, so
unlike PHP, Python, Node and C#, these examples cannot have zero dependencies.
They have one.

`ureq` with the `json` feature covers both, because it re-exports serde_json:
`ureq::json!` builds the body and `ureq::serde_json::Value` reads the response,
and `Cargo.toml` stays at a single dependency line.

It is blocking on purpose. `reqwest` would pull in an async runtime, and an
example that opens with `#[tokio::main]` teaches the runtime before it teaches
the API.

No second crate. No `anyhow`, no `clap`, no `dotenvy`. If something feels like
it needs another crate, it does not belong in an example.

## Rule two: the examples live under examples/v3/ and are declared by hand

Cargo only discovers examples directly under `examples/`. Ours are one level
down, because a version is a folder here as it is in every other language
repository, so each is declared in `Cargo.toml`:

```toml
[[example]]
name = "send-bulk"
path = "examples/v3/send-bulk.rs"
```

The name is the documentation page slug. Adding a method means adding the file
**and** the `[[example]]` block; forgetting the second is the usual mistake, and
the symptom is `cargo run --example <name>` saying it does not exist.

## Rule three: the examples are the documentation

Each file carries `// docs:start` and `// docs:end`. The region between them is
lifted verbatim into the method's page on docs.payam-resan.com, so it is read by
people who have never seen this repository.

Two consequences:

- **Full-line comments are stripped** when the region is lifted. Anything the
  reader must see has to be code. The `Success` check is an `if`, not a note.
- A path with two variants gets two files, the plain name for `POST` and a
  `-get` suffix for `GET`.

The full contract lives in the `handbook` repository, section `docs-site`, file
`code-samples.md`.

## Rule four: check Success, not the status code

The service answers `200` to everything, including a wrong key and an empty
account, so the HTTP status says nothing about whether the message was sent:

```rust
if response["Success"] != true {
    eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
    std::process::exit(1);
}
```

Worth knowing: ureq turns a non-2xx status into an `Err` by itself, so a wrong
URL surfaces as a transport error naming the status rather than reaching that
check. Both paths end in a readable message, which is the point.

## Rule five: a version is a folder

A new service version means a new `examples/v<n>/` and new `[[example]]` blocks.
No file inside an existing version folder is moved or renamed; older versions
still have users.

## Secrets

The key comes from `PAYAM_RESAN_API_KEY` in the environment. No key, no real
phone number and no customer name goes into a file here, not even a dead one.
Example numbers are `9121112222` upward and the example key is
`123456-XXXXXXXXXXXXXXX`.

## Layout

| Path | What it holds |
|---|---|
| `Cargo.toml` | the one dependency, and one `[[example]]` block per method |
| `examples/v3/` | one self-contained file per service operation |
| `.env.example` | the environment variables the examples read |

`Cargo.lock` is not committed. This is a set of examples, not an application,
and pinning the lock file would only go stale.

## Before every commit

```bash
cargo build --examples
for f in examples/v3/*.rs; do n=$(basename "$f" .rs); cargo run --example "$n" || echo "FAILED $n"; done
```

Point them at `api/V3SandBox/` first so no real message goes out.

## Git

Semantic messages, `type(scope): subject`, with no explanatory body and no
attribution trailer. Commits here are authored as Payam Resan.
