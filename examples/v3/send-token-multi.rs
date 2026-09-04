// SendTokenMulti - یک قالب، چند گیرنده، مقادیر متفاوت.
//
// پارامترها اینجا آرایه‌اند، نه p1 تا p10. درایه اول به {1} می‌نشیند، دومی به
// {2} و همین‌طور تا آخر: ترتیب از شماره جای‌گاه می‌آید، نه از جایی که در متن
// قالب دیده می‌شود.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example send-token-multi

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // قالب نمونه: «مرسوله شما از {2} تحویل پست شد. بارکد مرسوله پستی: {1}»
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
        "TemplateKey": "postcode",
        "Recipients": [
            {
                "Destination": 9121112222i64,
                "UserTraceId": 1001,
                "Parameters": ["BARCODE-AAA", "شیراز"],
            },
            {
                "Destination": 9121113333i64,
                "UserTraceId": 1002,
                "Parameters": ["BARCODE-BBB", "تبریز"],
            },
        ],
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/SendTokenMulti")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        println!(
            "{} => {}",
            message["UserTraceId"],
            message["FinalText"].as_str().unwrap_or("")
        );
    }

    Ok(())
}
// docs:end
