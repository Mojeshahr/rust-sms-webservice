// AccountInfo - اعتبار باقی‌مانده و خطوط فعال حساب.
//
// سبک‌ترین متد سرویس و بهترین راه آزمودن کلید: چیزی ارسال نمی‌کند، اعتباری
// مصرف نمی‌کند، و حتی با اعتبار صفر هم جواب می‌دهد.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example account-info

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/AccountInfo")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    println!("اعتبار: {}", response["Result"]["Credit"]);

    for line in response["Result"]["AvailableSenders"].as_array().unwrap_or(&vec![]) {
        println!("خط: {}", line);
    }

    Ok(())
}
// docs:end
