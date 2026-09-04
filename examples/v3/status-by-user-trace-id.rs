// StatusByUserTraceId - وضعیت پیامک با شناسه‌هایی که خودتان داده‌اید.
//
// اگر UserTraceId را کلید رکورد پایگاه داده خودتان بگذارید، دیگر لازم نیست Id
// سامانه را ذخیره کنید. این متد راه امن تشخیص ارسال تکراری هم هست: بعد از قطع
// ارتباط، اول اینجا بپرسید ثبت شده یا نه.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example status-by-user-trace-id

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
        "UserTraceIds": [1001i64, 1002i64],
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/StatusByUserTraceId")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        // کد ۸ یعنی این شناسه در حساب شما نیست. بعد از یک timeout، همین یعنی
        // ارسال ثبت نشده و می‌توانید با خیال راحت دوباره بفرستید.
        if message["StatusCode"] == 8 {
            println!("{}: ثبت نشده", message["UserTraceId"]);
            continue;
        }

        println!("{}: {}", message["UserTraceId"], message["Status"].as_str().unwrap_or(""));
    }

    Ok(())
}
// docs:end
