// SendMultiple - متن و خط فرستنده جدا برای هر گیرنده.
//
// برای پیام‌های شخصی‌سازی‌شده که با یک قالب ثابت پوشش داده نمی‌شوند. برخلاف
// SendBulk، اینجا Text و Sender در سطح هر گیرنده تعریف می‌شوند.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... PAYAM_RESAN_SENDER=... cargo run --example send-multiple

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sender: i64 = std::env::var("PAYAM_RESAN_SENDER")?.parse()?;

    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
        "Recipients": [
            {
                "Sender": sender,
                "Destination": 9121112222i64,
                "Text": "آقای محمدی، سفارش شما ارسال شد.",
                "UserTraceId": 1001,
            },
            {
                "Sender": sender,
                "Destination": 9121113333i64,
                "Text": "خانم رضایی، سفارش شما ارسال شد.",
                "UserTraceId": 1002,
            },
        ],
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/SendMultiple")
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
