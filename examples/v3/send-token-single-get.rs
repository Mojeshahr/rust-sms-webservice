// SendTokenSingle با GET - همان ارسال قالب، با ورودی در نشانی.
//
// برای آزمایش دستی مناسب است، برای محیط عملیاتی نه: در GET هم کلید حساب و هم
// مقدار رمز یک‌بارمصرف داخل نشانی می‌نشینند و در لاگ وب‌سرور و هدر Referer
// ثبت می‌شوند. واریانت POST را بردارید.
//
// جز ureq به چیزی وابسته نیست. Rust نه کلاینت HTTP دارد و نه JSON، و این تنها
// وابستگی نمونه‌هاست.
//
//   PAYAM_RESAN_API_KEY=... cargo run --example send-token-single-get

// docs:start
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("PAYAM_RESAN_API_KEY")?;

    let response: ureq::serde_json::Value =
        ureq::get("https://api.sms-webservice.com/api/V3/SendTokenSingle")
            .query("ApiKey", &api_key)
            .query("TemplateKey", "verifycode")
            .query("Destination", "9121112222")
            .query("p1", "123456")
            .call()?
            .into_json()?;

    if response["Success"] != true {
        eprintln!("ناموفق. کد {}: {}", response["ErrorCode"], response["Error"]);
        std::process::exit(1);
    }

    for message in response["Result"].as_array().unwrap_or(&vec![]) {
        println!(
            "شناسه {}، متن نهایی: {}",
            message["Id"],
            message["FinalText"].as_str().unwrap_or("")
        );
    }

    Ok(())
}
// docs:end
