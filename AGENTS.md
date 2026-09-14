# Working with this repository

You are looking at runnable Rust examples for the **Payam Resan** SMS web
service (`api.sms-webservice.com`, API V3), an Iranian SMS provider. Someone is
probably asking you to add SMS to their project.

Copy the example that matches the method, adapt it, and keep the rules below.
They are not style preferences — each one is a bug that this service produces if
you ignore it.

## Start here

```bash
export PAYAM_RESAN_API_KEY='123456-XXXXXXXXXXXXXXX'
cargo run --example account-info
```

Note the name has no extension. `account-info` sends nothing, spends no credit,
and answering proves the key and the connection.

Edition 2021. One dependency: `ureq` with the `json` feature.

## Rule 1: `ureq`, blocking, and why not `reqwest`

`ureq` is blocking on purpose. `reqwest` would drag in an async runtime, and an
example that opens with `#[tokio::main]` teaches the runtime before it teaches
the API. The `json` feature makes `ureq` re-export `serde_json`, so
`ureq::json!` and `ureq::serde_json::Value` come from one crate.

If the user's project is already async on `reqwest`, keep the payload and the
`Success` check and change only the sending layer.

## Rule 2: `cargo run --example <name>` and the `[[example]]` blocks

Cargo only discovers examples directly under `examples/`. These live one level
down in `examples/v3/`, because a version is a folder here as in every other
language repository — so each one is declared by hand in `Cargo.toml`.

If you add a method, add the file **and** the `[[example]]` block. Forgetting
the second is the usual mistake, and the symptom is
`cargo run --example <name>` saying it does not exist.

## Rule 3: `Success`, never the HTTP status

The service answers `200` to everything, including a wrong key, an empty account
and a malformed body.

```rust
if response["Success"] != true {
    eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
    std::process::exit(1);
}
```

The comparison is against `serde_json::Value`, so a missing field and an
explicit `false` both fail the check — which is what you want.

There is one wrinkle specific to `ureq`: it turns a non-2xx status into an `Err`
by itself, so a wrong URL surfaces as a transport error naming the status rather
than reaching this check.

`ErrorCode` is only meaningful when `Success` is false.

## Rule 4: the `i64` suffixes are load-bearing

A ten-digit Iranian mobile number overflows `i32`, and so does a message id:

```rust
"Destination": 9121112222i64,
"Ids": [9903211i64, 9903212i64],
```

Read them back with `.as_i64()`, never `.as_u32()` or a cast that narrows.

The examples deserialize into `ureq::serde_json::Value` rather than typed
structs, because a struct per method would put twenty lines of declarations in
front of four lines of API. In a real project, typed structs with `serde` are
the better choice — keep `i64` for every id.

## Rule 5: set a timeout

The examples rely on `ureq`'s defaults and set none. That is acceptable for a
one-shot example and not acceptable in a service. Build an agent with explicit
timeouts and reuse it:

```rust
let agent = ureq::AgentBuilder::new()
    .timeout(std::time::Duration::from_secs(30))
    .build();
```

A timeout is an *unclear* outcome, not a failure — see Rule 8 before retrying.

## Rule 6: the key never leaves the server

`std::env::var("PAYAM_RESAN_API_KEY")?`. Never compile it into the binary: a
string in an executable is trivially extracted.

Avoid `send` and `send-token-single-get` in production. Those are the `GET`
methods, where the key sits in the URL and lands in the web server log and the
`Referer` header.

## Rule 7: pick the right method

| The user wants | Use | Example |
|---|---|---|
| one text to many people | `SendBulk` | `send-bulk` |
| a different text per person | `SendMultiple` | `send-multiple` |
| a one-time password or code | `SendTokenSingle` | `send-token-single` |
| a template to many people | `SendTokenMulti` | `send-token-multi` |
| delivery status | `StatusByUserTraceId` | `status-by-user-trace-id` |
| balance and sender lines | `AccountInfo` | `account-info` |

**A one-time password goes through a template**, not free text — that is the
usual route for OTP, and the template fixes the sender line, which is why
`SendTokenSingle` takes no `Sender`. `token-list` lists the account's templates;
`Status` `2` means approved and sendable, `1` awaiting review, `3` rejected.

## Rule 8: phone numbers, the 99 cap, and `UserTraceId`

The service wants `9121112222` or `989121112222`. Users type `09121112222` or
`+989121112222`. Normalise before sending, or you get error `13`. Ninety-nine
recipients per request is the ceiling for `SendBulk`, `SendMultiple` and
`SendTokenMulti`.

Always send a `UserTraceId` — the user's own database id. After a timeout or
error `100`, resending blind may send twice; `StatusByUserTraceId` is the only
safe way to learn whether the message was registered. `StatusCode` of `8` there
means the id is not in the account, so it is safe to send again.

`SendTokenSingle` is the exception: it has no such input, so its `UserTraceId`
comes back null. If a trace id is needed for an OTP, use `SendTokenMulti` with a
single recipient.

`.query()` encodes exactly once — pre-encode as well and the message arrives
full of `%D8`.

## Rule 9: know which errors are worth retrying

These never succeed on retry — fix the cause; retrying only burns the rate limit
until the account hits error `20`:

`1`, `2`, `3`, `6`, `8`, `9`, `10`, `11`, `12`, `13`, `14`, `19`

`19` is an empty balance; `10` means the caller's IP is not on the account's
allowlist. Treat any unknown code the way you treat `100`: unclear outcome,
check with `StatusByUserTraceId` before resending.

## Rule 10: delivery status is a poll, not a callback

Status codes `0`, `1`, `2`, `3` and `10` mean still in flight — query again
later, and not more often than every few minutes or you will hit error `20`.
Everything else is final. Branch on `StatusCode`, never on the `Status` text,
which is Persian prose meant for humans and can change.

## Rule 11: `GetInbox` consumes what it returns

The service hands over each incoming message **once**. Never call it from a
request handler: every request consumes unread messages permanently. It belongs
in a scheduled task that writes straight to storage.

The sender field is called `Form`, not `From`. That is the service's spelling.

## Testing without spending credit

Replace `V3` with `V3SandBox` in the URL. No message is sent and no credit is
spent. `TokenList` is not implemented there and answers `404`, which `ureq`
surfaces as a transport error.

The sandbox is a simulator, not a mirror of the account: credit is always
`1234567`, sender lines are invented, and **it accepts any key**. Success there
proves nothing about the user's real key.

## Where the authoritative answers are

- Method reference and error tables: <https://docs.payam-resan.com>
- Machine-readable OpenAPI: <https://github.com/Mojeshahr/sms-webservice-spec>

If the spec and these examples ever disagree, the spec wins — report it as a bug
rather than guessing.
