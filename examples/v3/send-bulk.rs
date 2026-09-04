// SendBulk - یک متن به چند گیرنده، هر کدام با شناسه پی‌گیری خودتان.
//
// روش پیشنهادی برای ارسال عملیاتی. کلید در بدنه درخواست می‌رود نه در نشانی،
// و برای هر گیرنده UserTraceId می‌پذیرد تا گزارش تحویل را بدون نگه‌داشتن Id
// سامانه بگیرید.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... PAYAM_RESAN_SENDER=... cargo run --example send-bulk

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sender: i64 = std::env::var("PAYAM_RESAN_SENDER")?.parse()?;

    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
        "Sender": sender,
        "Text": "سفارش شما ثبت شد.",
        "Recipients": [
            { "Destination": 9121112222i64, "UserTraceId": 1001 },
            { "Destination": 9121113333i64, "UserTraceId": 1002 },
        ],
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/SendBulk")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        println!("{} => شناسه {}", message["UserTraceId"], message["Id"]);
    }

    Ok(())
}
// docs:end
