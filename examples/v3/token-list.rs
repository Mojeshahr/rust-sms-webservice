// TokenList - قالب‌های حساب، با کلید و متن و وضعیت تأییدشان.
//
// برای پیدا کردن TemplateKey که متدهای ارسال قالب لازم دارند. این متد هم مثل
// AccountInfo از بررسی اعتبار معاف است.
//
// روی سرور آزمایشی پیاده نشده و ۴۰۴ می‌دهد؛ همین متد را از سرور عملیاتی صدا
// بزنید، چیزی نمی‌فرستد و اعتباری مصرف نمی‌کند.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example token-list

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/TokenList")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for template in response["Result"].as_array().unwrap_or(&vec![]) {
        let sendable = if template["Status"] == 2 { "قابل ارسال" } else { "قابل ارسال نیست" };
        println!(
            "{} ({}): {}",
            template["Key"].as_str().unwrap_or(""),
            sendable,
            template["TextTemplate"].as_str().unwrap_or("")
        );
    }

    Ok(())
}
// docs:end
