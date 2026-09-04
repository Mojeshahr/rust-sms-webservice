// StatusById - وضعیت پیامک با شناسه‌هایی که متد ارسال برگردانده است.
//
// دسته‌ای بپرسید، نه یکی‌یکی. فاصله استعلام‌ها را هم کمتر از چند دقیقه
// نگذارید، وگرنه به خطای ۲۰ می‌خورید.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example status-by-id

// docs:start
use std::error::Error;

// شرط را روی StatusCode بگذارید، نه روی متن Status. این پنج کد یعنی هنوز در
// راه است و باید بعداً دوباره استعلام کنید، نه اینکه دوباره بفرستید.
const PENDING: [i64; 5] = [0, 1, 2, 3, 10];

fn main() -> Result<(), Box<dyn Error>> {
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
        "Ids": [9903211i64, 9903212i64],
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/StatusById")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        let code = message["StatusCode"].as_i64().unwrap_or(-1);
        let again = if PENDING.contains(&code) { " (بعداً دوباره بپرسید)" } else { "" };
        println!("{}: {}{}", message["Id"], message["Status"].as_str().unwrap_or(""), again);
    }

    Ok(())
}
// docs:end
