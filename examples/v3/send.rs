// Send - ساده‌ترین ارسال، یک متن به چند شماره با یک درخواست GET.
//
// برای آزمایش سریع خوب است. در محیط عملیاتی SendBulk را بردارید: کلید را از
// نشانی بیرون می‌برد و برای هر گیرنده شناسه پی‌گیری می‌پذیرد.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... PAYAM_RESAN_SENDER=... cargo run --example send

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("PAYAM_RESAN_API_KEY")?;
    let sender = std::env::var("PAYAM_RESAN_SENDER")?;

    // متد query دقیقاً یک بار encode می‌کند. اگر متن را خودتان هم پیش از این
    // encode کنید، پیامک با نویسه‌های %D8 به گوشی می‌رسد.
    let response: ureq::serde_json::Value =
        ureq::get("https://api.sms-webservice.com/api/V3/Send")
            .query("ApiKey", &api_key)
            .query("Sender", &sender)
            .query("Text", "کد تأیید شما ۱۲۳۴۵۶ است")
            .query("Recipients", "9121112222,9121113333")
            .call()?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        println!("شناسه {}", message["Id"]);
    }

    Ok(())
}
// docs:end
