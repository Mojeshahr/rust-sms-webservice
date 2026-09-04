// GetInbox - پیامک‌هایی که کاربران به خطوط حساب شما فرستاده‌اند.
//
// این یک استعلام است، نه webhook: سامانه چیزی به سرور شما نمی‌فرستد و باید
// خودتان دوره‌ای صدایش بزنید. فاصله را کمتر از چند دقیقه نگذارید، وگرنه به
// خطای ۲۰ می‌خورید.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example get-inbox

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/GetInbox")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for sms in response["Result"].as_array().unwrap_or(&vec![]) {
        // نام فیلد فرستنده در خود سرویس Form است، نه From. دنبال From نگردید.
        println!(
            "{}  {} -> {}: {}",
            sms["Time"].as_str().unwrap_or(""),
            sms["Form"],
            sms["To"],
            sms["Text"].as_str().unwrap_or("")
        );
    }

    Ok(())
}
// docs:end
