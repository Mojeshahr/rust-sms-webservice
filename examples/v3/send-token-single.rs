// SendTokenSingle - ارسال قالب به یک شماره، با بدنه JSON.
//
// مسیر معمول رمز یک‌بارمصرف. خط فرستنده ورودی ندارد؛ سامانه آن را از روی خود
// قالب برمی‌دارد. همین واریانت POST را به کار ببرید، نه GET: در GET هم کلید
// حساب و هم خود رمز داخل نشانی و لاگ وب‌سرور می‌نشینند.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example send-token-single

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let payload = ureq::json!({
        "ApiKey": std::env::var("PAYAM_RESAN_API_KEY")?,
        "TemplateKey": "verifycode",
        "Destination": 9121112222i64,
        "p1": "123456",
    });

    let response: ureq::serde_json::Value =
        ureq::post("https://api.sms-webservice.com/api/V3/SendTokenSingle")
            .send_json(payload)?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    // این متد UserTraceId در ورودی ندارد، پس در پاسخ null برمی‌گردد. اگر شناسه
    // پی‌گیری لازم دارید، SendTokenMulti را حتی برای یک گیرنده هم می‌شود به کار برد.
    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        println!("شناسه {} از خط {}", message["Id"], message["Sender"]);
        println!("متن نهایی: {}", message["FinalText"].as_str().unwrap_or(""));
    }

    Ok(())
}
// docs:end
