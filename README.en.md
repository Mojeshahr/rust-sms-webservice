<div align="center">

<a href="https://payam-resan.com">
  <img src=".github/assets/logo.svg" width="64" height="64" alt="Payam Resan">
</a>

<h1>Rust examples for the Payam Resan SMS web service</h1>

Talk to the <a href="https://payam-resan.com"><b>Payam Resan SMS panel</b></a> from Rust<br>
One runnable file per API method

[![API](https://img.shields.io/badge/API-V3-0a7cbd)](https://payam-resan.com)
[![Rust](https://img.shields.io/badge/Rust-2021-dea584)](https://www.rust-lang.org)
[![Dependency](https://img.shields.io/badge/dependency-ureq-2ea44f)](https://crates.io/crates/ureq)
[![License](https://img.shields.io/badge/license-MIT-6e7781)](LICENSE)

<a href="README.md">فارسی</a> · <b>English</b>

</div>

<sub>Looking for another language? The same examples exist for the others at
[github.com/Mojeshahr](https://github.com/Mojeshahr).</sub>

---

## Quick start

```bash
git clone https://github.com/Mojeshahr/rust-sms-webservice.git
cd rust-sms-webservice

export PAYAM_RESAN_API_KEY='123456-XXXXXXXXXXXXXXX'
export PAYAM_RESAN_SENDER='30004040'

cargo run --example account-info
```

Start with `account-info`. It sends nothing, spends no credit, and if it
answers then both the key and the connection are fine.

## Why one dependency

Rust has neither an HTTP client nor a JSON parser in its standard library, so
unlike the PHP, Python, Node and C# examples these cannot have none. They have
one.

`ureq` with the `json` feature covers both, because it re-exports serde_json:
`ureq::json!` builds the body and `ureq::serde_json::Value` reads the response.

It is blocking on purpose. `reqwest` would pull in an async runtime, and an
example opening with `#[tokio::main]` teaches the runtime before it teaches the
API.

## Before sending anything real

There is a sandbox server that answers exactly like production but sends no
message and spends no credit. Swap `V3` for `V3SandBox` in the URL. The one
exception is `TokenList`, which the sandbox does not implement.

## The methods

| Example | Method | What it does |
|---|---|---|
| [account-info.rs](examples/v3/account-info.rs) | `AccountInfo` | Credit and active lines |
| [send.rs](examples/v3/send.rs) | `Send` | Simple send over `GET` |
| [send-bulk.rs](examples/v3/send-bulk.rs) | `SendBulk` | One text to many recipients, with tracking ids |
| [send-multiple.rs](examples/v3/send-multiple.rs) | `SendMultiple` | A separate text per recipient |
| [token-list.rs](examples/v3/token-list.rs) | `TokenList` | The account's templates |
| [send-token-single.rs](examples/v3/send-token-single.rs) | `SendTokenSingle` | Send a template to one number |
| [send-token-single-get.rs](examples/v3/send-token-single-get.rs) | `SendTokenSingle` | The same, over `GET` |
| [send-token-multi.rs](examples/v3/send-token-multi.rs) | `SendTokenMulti` | One template, many recipients |
| [status-by-id.rs](examples/v3/status-by-id.rs) | `StatusById` | Status by the service's id |
| [status-by-user-trace-id.rs](examples/v3/status-by-user-trace-id.rs) | `StatusByUserTraceId` | Status by your own id |
| [get-inbox.rs](examples/v3/get-inbox.rs) | `GetInbox` | Messages people sent to your lines |

Run any of them with `cargo run --example <name>`, without the extension.

## Using this in your own project

The examples depend on nothing in this repository, so copying the body of `main`
into your own code is enough. Add the dependency with:

```toml
[dependencies]
ureq = { version = "2", features = ["json"] }
```

If your project is async and already uses `reqwest`, change only the sending
layer; the request shape and the response check are the same.

## Things that will save you time

**Do not read the HTTP status code.** The service answers `200` to everything,
including a wrong key. Decide on the `Success` field.

**ureq turns a non-2xx status into an error by itself.** A wrong URL therefore
surfaces as a transport error naming the status, rather than reaching the
`Success` check. Both paths end in a readable message.

**Recipient numbers carry no leading zero.** Use `9121112222`, or
`989121112222` with the country code. A number that does not start with `9` or
`989` returns error code `13`.

**Do not encode the text twice.** In `send.rs`, `query` already does it once.
Encode it yourself beforehand and the message arrives full of `%D8`.

**Send a unique `UserTraceId` per recipient.** After a timeout it is the only
way to learn whether the message was registered.

## Key safety

The key is a secret. It does not belong in a code repository, in browser
JavaScript, or in a mobile app bundle. It belongs in an environment variable,
which is where every example here reads it from.

If a key leaks, issue a new one from the panel. A deleted key never comes back.

## Layout

| Path | What it holds |
|---|---|
| `Cargo.toml` | The one dependency, and one `[[example]]` block per method |
| `examples/v3/` | One self-contained example per service operation |
| `.env.example` | The environment variables the examples read |

The `v3` in the path is deliberate. A new service version means a new
`examples/v<n>/`, with the existing folder left alone.

## Documentation and support

The full guide to the web service is at
[docs.payam-resan.com](https://docs.payam-resan.com), and the machine-readable
OpenAPI description is in
[sms-webservice-spec](https://github.com/Mojeshahr/sms-webservice-spec).

Question or bug? [Open an issue](https://github.com/Mojeshahr/rust-sms-webservice/issues)
or contact [support](https://payam-resan.com).

## License

Released under the MIT license. Full text in [`LICENSE`](LICENSE).

<br>
<div align="center">
  <sub>
    <img src=".github/assets/logo.svg" width="16" height="16" alt="" align="top">
    &nbsp;<b>Payam Resan SMS Panel - Moje Shahr</b>&nbsp;
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset=".github/assets/mojeshahr-dark.svg">
      <img src=".github/assets/mojeshahr-light.svg" width="16" height="16" alt="" align="top">
    </picture>
  </sub>
  <br>
  <sub>
    <a href="https://payam-resan.com">payam-resan.com</a>
    &nbsp;·&nbsp;
    <a href="https://mojeshahr.ir">mojeshahr.ir</a>
  </sub>
</div>
