<div align="center">

<a href="https://payam-resan.com">
  <img src=".github/assets/logo.svg" width="64" height="64" alt="پیام رسان">
</a>

<h1>نمونه‌کدهای Rust وب‌سرویس پیام رسان</h1>

اتصال به وب‌سرویس <a href="https://payam-resan.com"><b>پنل پیامکی پیام رسان</b></a> با Rust<br>
یک فایل قابل اجرا به‌ازای هر متد سرویس

[![API](https://img.shields.io/badge/API-V3-0a7cbd)](https://payam-resan.com)
[![Rust](https://img.shields.io/badge/Rust-2021-dea584)](https://www.rust-lang.org)
[![Dependency](https://img.shields.io/badge/dependency-ureq-2ea44f)](https://crates.io/crates/ureq)
[![License](https://img.shields.io/badge/license-MIT-6e7781)](LICENSE)

<b>فارسی</b> · <a href="README.en.md">English</a>

</div>

<sub>دنبال زبان دیگری هستید؟ همین نمونه‌ها برای زبان‌های دیگر هم در
[github.com/Mojeshahr](https://github.com/Mojeshahr) هست.</sub>

---

## شروع سریع

```bash
git clone https://github.com/Mojeshahr/rust-sms-webservice.git
cd rust-sms-webservice

export PAYAM_RESAN_API_KEY='123456-XXXXXXXXXXXXXXX'
export PAYAM_RESAN_SENDER='30004040'

cargo run --example account-info
```

با `account-info` شروع کنید: چیزی ارسال نمی‌کند، اعتباری مصرف نمی‌کند، و اگر
جواب داد یعنی کلید و اتصال هر دو سالم‌اند.

## چرا یک وابستگی

Rust نه کلاینت HTTP در کتابخانه استانداردش دارد و نه JSON، پس برخلاف نمونه‌های
PHP و Python و Node و C# نمی‌شود صفر وابستگی داشت. یکی دارد.

`ureq` با ویژگی `json` هر دو را می‌دهد، چون خودش `serde_json` را دوباره صادر
می‌کند: `ureq::json!` بدنه را می‌سازد و `ureq::serde_json::Value` پاسخ را
می‌خواند.

عمداً blocking است. `reqwest` یک async runtime با خودش می‌آورد، و نمونه‌ای که با
`#[tokio::main]` شروع شود اول runtime را یاد می‌دهد نه API را.

## پیش از ارسال واقعی

یک سرور آزمایشی هست که مثل سرور عملیاتی جواب می‌دهد ولی پیامکی نمی‌فرستد و
اعتباری مصرف نمی‌کند. کافی است `V3` در نشانی را با `V3SandBox` عوض کنید. تنها
استثنا `TokenList` است که روی آن سرور پیاده نشده.

## متدها

<div dir="rtl">

| نمونه | متد | کار |
|---|---|---|
| [account-info.rs](examples/v3/account-info.rs) | `AccountInfo` | اعتبار و خطوط فعال |
| [send.rs](examples/v3/send.rs) | `Send` | ارسال ساده با `GET` |
| [send-bulk.rs](examples/v3/send-bulk.rs) | `SendBulk` | یک متن به چند گیرنده، با شناسه پی‌گیری |
| [send-multiple.rs](examples/v3/send-multiple.rs) | `SendMultiple` | متن جدا برای هر گیرنده |
| [token-list.rs](examples/v3/token-list.rs) | `TokenList` | فهرست قالب‌ها |
| [send-token-single.rs](examples/v3/send-token-single.rs) | `SendTokenSingle` | ارسال قالب به یک شماره |
| [send-token-single-get.rs](examples/v3/send-token-single-get.rs) | `SendTokenSingle` | همان، با `GET` |
| [send-token-multi.rs](examples/v3/send-token-multi.rs) | `SendTokenMulti` | یک قالب، چند گیرنده |
| [status-by-id.rs](examples/v3/status-by-id.rs) | `StatusById` | وضعیت با شناسه سامانه |
| [status-by-user-trace-id.rs](examples/v3/status-by-user-trace-id.rs) | `StatusByUserTraceId` | وضعیت با شناسه خودتان |
| [get-inbox.rs](examples/v3/get-inbox.rs) | `GetInbox` | پیامک‌های رسیده |

</div>

هر کدام با `cargo run --example <نام>` اجرا می‌شوند، بدون پسوند.

## استفاده در پروژه خودتان

نمونه‌ها به هیچ چیز این مخزن وابسته نیستند، پس کپی‌کردن بدنه `main` داخل کد
خودتان کافی است. وابستگی را این‌طور اضافه کنید:

```toml
[dependencies]
ureq = { version = "2", features = ["json"] }
```

اگر پروژه‌تان async است و `reqwest` دارد، فقط لایه ارسال را عوض کنید؛ شکل
درخواست و بررسی پاسخ همان است.

## چند نکته که وقت‌تان را می‌خرد

**کد وضعیت HTTP را نخوانید.** سرویس همیشه `200` برمی‌گرداند، حتی وقتی کلید
اشتباه است. تصمیم را از فیلد `Success` بگیرید.

**`ureq` وضعیت غیر ۲xx را خودش خطا حساب می‌کند.** یعنی نشانی اشتباه به‌جای
رسیدن به بررسی `Success`، یک خطای گویا با کد وضعیت می‌دهد. هر دو مسیر به پیام
خوانا می‌رسند.

**شماره گیرنده صفر ابتدایی ندارد.** یعنی `9121112222` یا با کد کشور
`989121112222`. شماره‌ای که با `9` یا `989` شروع نشود کد خطای `13` می‌گیرد.

**متن را دوباره encode نکنید.** در `send.rs` متد `query` خودش یک بار این کار را
می‌کند. اگر پیش از آن هم encode کنید، پیامک با نویسه‌های `%D8` به گوشی می‌رسد.

**برای هر گیرنده یک `UserTraceId` یکتا بفرستید.** بعد از یک timeout، این تنها
راه فهمیدن این است که پیامک ثبت شده یا نه.

## امنیت کلید

کلید یک راز است. در مخزن کد، در جاوااسکریپت مرورگر و در بسته اپلیکیشن موبایل
نباید قرار بگیرد. جای آن متغیر محیطی است، همان‌طور که همه نمونه‌ها می‌خوانندش.

اگر کلیدی لو رفت، از پنل یکی تازه بسازید. کلید حذف‌شده برنمی‌گردد.

## ساختار

<div dir="rtl">

| مسیر | چه چیزی دارد |
|---|---|
| `Cargo.toml` | تنها وابستگی، و یک بلوک `[[example]]` به‌ازای هر متد |
| `examples/v3/` | یک نمونه مستقل به‌ازای هر عملیات سرویس |
| `.env.example` | نمونه متغیرهای محیطی |

</div>

عدد `v3` در مسیر عمدی است. نسخه تازه سرویس یعنی پوشه `examples/v<n>/` تازه، و
پوشه موجود دست‌نخورده می‌ماند.

## مستندات و پشتیبانی

راهنمای کامل وب‌سرویس در [docs.payam-resan.com](https://docs.payam-resan.com)
است. توصیف ماشین‌خوان OpenAPI هم در
[sms-webservice-spec](https://github.com/Mojeshahr/sms-webservice-spec).

## مجوز

MIT. متن کامل در [`LICENSE`](LICENSE).
