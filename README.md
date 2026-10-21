# AllyMailer Rust SDK

[![Rust](https://img.shields.io/crates/v/allymailer-rust-sdk)](https://crates.io/crates/allymailer-rust-sdk) [![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Офіційна бібліотека Rust для API транзакційних листів [AllyMailer](https://allymailer.com.ua).

[Довідник API](https://docs.allymailer.com.ua/ua/web-api-ref) · Документація для LLM: [llms.txt](https://docs.allymailer.com.ua/llms.txt)

> **axum?** Встановіть [`allymailer-axum`](https://github.com/allymailer/allymailer-axum): у ньому цей SDK, налаштування, поштовий транспорт і маршрут для вебхуків.

## Встановлення

```bash
cargo add allymailer-rust-sdk
cargo add tokio --features macros,rt-multi-thread
```

Вимоги: Rust 1.70+ (edition 2021), async runtime (tokio). Сторонніх залежностей немає.

Отримайте API-ключ у [панелі керування](https://cp.allymailer.com) і зберігайте його у змінній оточення, а не в коді:

```bash
export ALLYMAILER_API_KEY="your-api-key"
```

## Надсилання листа

```rust
use allymailer::api::email::model::EmailSendRequest;
use allymailer::api::email::model::MessageObject;
use allymailer::api::email::model::MessageObjectRecipientsItem as Recipient;
use allymailer::common::model::BodyObject;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("ALLYMAILER_API_KEY").expect("set ALLYMAILER_API_KEY");
    let client = AllyMailerClient::new(api_key);

    let message = MessageObject {
        recipients: vec![Recipient::from_address("Олексій <alex@example.com>", None)],
        from_email: Some("shop@example.com".into()),
        from_name: Some("Магазин".into()),
        subject: "Ваше замовлення".into(),
        body: BodyObject::html("<p>Вітаємо, {{to_name}}! Ваше замовлення надіслано.</p>"),
        ..Default::default()
    };
    let response = client.email().send(&EmailSendRequest { message }).await?;
    println!("{}", response.job_id.unwrap_or_default());
    Ok(())
}
```

Адреса відправника має бути на домені, підтвердженому у вашому обліковому записі. Домену ще немає? Надішліть лист із [sandbox-домену](#без-свого-домену-тестовий-sandbox-домен) у три кроки.

## Зміст

- [Рецепти](#рецепти): персоналізація · вкладення · копія і прихована копія · відкладене надсилання · безпечний повтор · помилки · вебхуки
- [Без свого домену: тестовий sandbox-домен](#без-свого-домену-тестовий-sandbox-домен)
- [Запуск у продакшен](#запуск-у-продакшен)
- [Довідник](#довідник)
- [Допомога](#допомога)

## Рецепти

Кожен рецепт продовжує приклад вище: клієнт та імпорти вже є.

### Персоналізація для кожного одержувача

До 500 одержувачів в одному виклику, кожен отримує окрему копію зі своїми підстановками. Метадані одержувача повертаються в його подіях вебхука.

```rust
use std::collections::HashMap;

let message = MessageObject {
    recipients: vec![
        Recipient::from_address(
            "Олексій <alex@example.com>",
            Some(HashMap::from([("order".into(), "1001".into())])),
        )
        .with_metadata(HashMap::from([("user_id".into(), "42".into())])),
        Recipient::from_address(
            "Марія <maria@example.com>",
            Some(HashMap::from([("order".into(), "1002".into())])),
        )
        .with_metadata(HashMap::from([("user_id".into(), "43".into())])),
    ],
    from_email: Some("shop@example.com".into()),
    subject: "Замовлення №{{order}}".into(),
    body: BodyObject::html("<p>{{to_name}}, замовлення №{{order}} надіслано.</p>"),
    ..Default::default()
};
client.email().send(&EmailSendRequest { message }).await?;
```

### Вкладення

```rust
use allymailer::common::model::AttachmentsArrayItem;

let content = std::fs::read("invoice.pdf")?;
let invoice = AttachmentsArrayItem::from_bytes("application/pdf", "invoice.pdf", &content);
let message = MessageObject {
    recipients: vec![Recipient::from_address("alex@example.com", None)],
    from_email: Some("shop@example.com".into()),
    subject: "Рахунок".into(),
    body: BodyObject::html("<p>Рахунок у вкладенні.</p>"),
    attachments: Some(vec![invoice]),
    ..Default::default()
};
client.email().send(&EmailSendRequest { message }).await?;
```

### Копія і прихована копія

Усі адреси передаються в `recipients` (кожна отримує копію), видимі — ще й у заголовках. `audit@example.com` немає в жодному заголовку: це прихована копія. Правила й обмеження: [копія і прихована копія](https://docs.allymailer.com.ua/ua/cc-and-bcc).

```rust
use allymailer::common::model::HeadersObject;
use std::collections::HashMap;

let message = MessageObject {
    recipients: vec![
        Recipient::from_address("alex@example.com", None),
        Recipient::from_address("boss@example.com", None),
        Recipient::from_address("audit@example.com", None),
    ],
    from_email: Some("shop@example.com".into()),
    subject: "Звіт за тиждень".into(),
    body: BodyObject::html("<p>Звіт у вкладенні.</p>"),
    headers: Some(HeadersObject {
        extra: HashMap::from([
            ("To".into(), "alex@example.com".into()),
            ("CC".into(), "boss@example.com".into()),
        ]),
        ..Default::default()
    }),
    ..Default::default()
};
client.email().send(&EmailSendRequest { message }).await?;
```

### Відкладене надсилання

`send_at` — в UTC, не далі ніж на 24 години вперед.

```rust
use allymailer::api::email::model::MessageObjectOptions;
use allymailer::client::utc_datetime;
use std::time::Duration;
use std::time::SystemTime;

// UTC, "YYYY-MM-DD hh:mm:ss"
let send_at = utc_datetime(SystemTime::now() + Duration::from_secs(3600));
let message = MessageObject {
    recipients: vec![Recipient::from_address("alex@example.com", None)],
    from_email: Some("shop@example.com".into()),
    subject: "Нагадування".into(),
    body: BodyObject::text("До зустрічі завтра!"),
    options: Some(MessageObjectOptions {
        send_at: Some(send_at),
        ..Default::default()
    }),
    ..Default::default()
};
client.email().send(&EmailSendRequest { message }).await?;
```

### Безпечний повтор

Повтор із тим самим `idempotence_key` протягом хвилини не надішлеться вдруге: з ним повторюйте мережеві помилки, таймаути, 429 і 5xx.

```rust
let message = MessageObject {
    recipients: vec![Recipient::from_address("alex@example.com", None)],
    from_email: Some("shop@example.com".into()),
    subject: "Скидання пароля".into(),
    body: BodyObject::html("<p>Посилання для скидання пароля.</p>"),
    idempotence_key: Some("password-reset-8f3a".into()),
    ..Default::default()
};
client.email().send(&EmailSendRequest { message }).await?;
```

### Обробка помилок

Одержувачі, яких API відкинув (відписані, неіснуючі, у стоп-листі), — не помилка: вони у `failed_emails` відповіді, решта лист отримає.

```rust
if let Err(e) = client.email().send(&request).await {
    // HTTP-статус (0: мережева помилка або таймаут), код помилки API, повідомлення
    eprintln!("{} {} {}", e.http_status, e.code.unwrap_or(0), e.message);
}
```

### Статуси доставки

Зареєструйте свою HTTPS-адресу один раз (`webhook/set`). Потім перевіряйте підпис кожного сповіщення (`auth`: MD5 сирого тіла, у якому значення `auth` замінено на API-ключ; на неправильний — 401) і обробляйте події.

```rust
// Cargo.toml: md5 = "0.7"
use allymailer::api::webhook::WebhookApi;

/// тіло POST-запиту як надійшло; повертає HTTP-статус відповіді
fn handle_webhook(raw_body: &str) -> u16 {
    let Ok(callback) = WebhookApi::parse_callback(raw_body) else {
        return 400;
    };
    // auth: MD5 сирого тіла, у якому значення auth замінено на ваш API-ключ
    let auth = callback.auth.unwrap_or_default();
    let key = std::env::var("ALLYMAILER_API_KEY").unwrap_or_default();
    let expected = format!("{:x}", md5::compute(raw_body.replacen(&auth, &key, 1)));
    if auth.is_empty() || expected != auth {
        return 401;
    }

    for user in callback.events_by_user.unwrap_or_default() {
        for event in user.events.unwrap_or_default() {
            // email, status (delivered, opened, hard_bounced, spam, ...), job_id, metadata
            println!("{:?}", event.event_data);
        }
    }
    200
}
```

> **Відповідайте 200 протягом 3 секунд**, а події обробляйте у фоні (через чергу): інакше AllyMailer повторює запит кожні 10 хвилин протягом доби (зростає `retry_count`), а обробник, на якому за добу не пройшов жоден запит хоча б для 10 подій, вимикається. Обробник має витримувати **щонайменше 5 паралельних запитів до 100 подій у кожному**; повтор надсилає ті самі події ще раз — прибирайте дублікати.

## Без свого домену: тестовий sandbox-домен

Поки ваш домен не підтверджено, листи можна надсилати із **sandbox-домену** — тестового домену, який є в кожного облікового запису: він уже підтверджений, DKIM і SPF налаштовані. Листи з нього доставляються лише на підтверджені вами адреси (посилання для підтвердження приходить листом і діє 48 годин), до 100 на день. Через 31 день після створення sandbox-домен видаляється, але його можна отримати знову: наступний виклик `domain/get-sandbox-domain` створить новий (з іншою назвою, тому беріть її з відповіді, а не прописуйте в коді).

```rust
use allymailer::api::confirmed_email::model::ConfirmedEmailRequest;

// 1. Ваш sandbox-домен: створюється під час першого виклику, далі повертається той самий
let sandbox = client.domain().get_sandbox_domain().await?;
let domain = sandbox.domain.unwrap_or_default();

// 2. Підтвердіть свою скриньку: AllyMailer надішле посилання, перейдіть за ним протягом 48 годин
let confirm = ConfirmedEmailRequest {
    email: "you@example.com".into(),
};
client.confirmed_email().create(&confirm).await?;

// 3. Надішліть із будь-якої адреси sandbox-домену на підтверджену адресу
let message = MessageObject {
    recipients: vec![Recipient::from_address("you@example.com", None)],
    from_email: Some(format!("noreply@{domain}")),
    subject: "Привіт від AllyMailer".into(),
    body: BodyObject::html("<p>Працює!</p>"),
    ..Default::default()
};
client.email().send(&EmailSendRequest { message }).await?;
```

<details>
<summary>Обмеження sandbox</summary>

- Надсилання лише на підтверджені адреси: до 10 на обліковий запис.
- До 100 листів на день.
- Sandbox-домен спливає через 31 день; знову викличте `domain/get-sandbox-domain`, щоб отримати новий.
- Працює лише з API-ключем облікового запису, не з ключем проєкту.
- Не показується в `domain/list`; інші методи `domain/*` до нього не застосовуються.

Sandbox — для розробки та тестування. Не використовуйте sandbox-домен як відправника на проді.

</details>

<details>
<summary>Те саме через curl</summary>

```bash
BASE=https://cp.allymailer.com/ua/transactional/api/v1

curl -s -X POST $BASE/domain/get-sandbox-domain.json \
  -H "Content-Type: application/json" -H "X-API-KEY: $ALLYMAILER_API_KEY" -d '{}'

curl -s -X POST $BASE/confirmed-email/create.json \
  -H "Content-Type: application/json" -H "X-API-KEY: $ALLYMAILER_API_KEY" \
  -d '{"email": "you@example.com"}'

curl -s -X POST $BASE/email/send.json \
  -H "Content-Type: application/json" -H "X-API-KEY: $ALLYMAILER_API_KEY" \
  -d '{
        "message": {
          "recipients": [
            {
              "email": "you@example.com"
            }
          ],
          "from_email": "noreply@<sandbox-domain>",
          "subject": "Привіт від AllyMailer",
          "body": {
            "html": "<p>Працює!</p>"
          }
        }
      }'
```

</details>

## Запуск у продакшен

**1. Підтвердіть домен надсилання.** `domain/get-dns-records` повертає запис підтвердження, DKIM-ключ і адресу для DMARC; додайте записи у свого DNS-провайдера, потім `domain/validate-verification-record` і `domain/validate-dkim`.

| Тип | Хост | Значення | Звідки взяти |
|---|---|---|---|
| TXT | `@` | `allymailer-validate-hash=…` | поле `verification-record` відповіді, як є |
| TXT | `<селектор>._domainkey` (селектор — на сторінці домену в панелі керування, наприклад `us`) | `v=DKIM1; k=rsa; p=<dkim>` | поле `dkim` відповіді після `v=DKIM1; k=rsa; p=` |
| TXT | `@` | `v=spf1 include:spf.allymailer.com ~all` | у відповіді немає; якщо SPF-запис уже є, додайте include до нього |
| CNAME | `_dmarc` | `<домен>.dmarc.…` | поле `dmarc` відповіді (DMARC-політику публікує AllyMailer); якщо в домену вже є свій TXT-запис DMARC, залиште його |

**2. Надійне надсилання.**

| | |
|---|---|
| Повтори | `429` і `5xx` з backoff і тим самим `idempotence_key`; інші помилки виправляйте, а не повторюйте |
| Ліміти | 500 одержувачів і 10 МБ на запит разом із вкладеннями: у base64 файл стає приблизно на третину більшим |
| Тіло | завжди `html` або `plaintext` (або із шаблону) |
| Статус доставки | вебхук, а не опитування |
| Черга | надсилайте з фонового завдання, а не всередині веб-запиту |
| Налаштування | `from_email` з конфігурації: перехід із sandbox на продакшен без змін у коді |
| Типові налаштування | не прибирайте блок відписки та трекінг; змінити їх можна через підтримку AllyMailer |

## Довідник

### Розділи

| Розділ | Методи | Опис |
|---|---|---|
| [Email](docs/Email.md) | `send`, `subscribe` | Ця група методів пов'язана з надсиланням email. |
| [EmailValidation](docs/EmailValidation.md) | `single` | Методи перевірки email-адрес. Перевірка адреси електронної пошти надає докладну інформацію про працездатність email. Це дозволить виключити з надсилання неіснуючі email, знизити показник скарг, очистити список підписників і врешті-решт покращити вашу репутацію як відправника. Наприклад, ви можете інтегрувати процес перевірки у форму реєстрації вашого сайту і вже на етапі введення адреси повідомляти користувачам, що адреса, яку вони ввели у форму, недійсна. |
| [ConfirmedEmail](docs/ConfirmedEmail.md) | `create`, `list`, `delete` | Керування email-адресами, підтвердженими для надсилання із sandbox-домену. Методи доступні лише з API-ключем основного облікового запису. |
| [Template](docs/Template.md) | `set`, `get`, `list`, `delete` | Ви можете зберегти шаблон на боці AllyMailer і потім посилатися на нього за ідентифікатором. Це не обов'язково, але вкрай рекомендовано у разі повторюваних email-повідомлень або за великих обсягів надсилання. AllyMailer підтримує два різні синтаксиси для шаблонів, докладніше читайте [тут](https://docs.allymailer.com.ua/ua/template-engines). |
| [Webhook](docs/Webhook.md) | `set`, `get`, `list`, `delete` | AllyMailer дозволяє вам отримувати сповіщення про зміну статусу доставки листа або спам-блокування. Ви надаєте URL, і в разі виникнення події, що вас цікавить, ми сповіщаємо вас, передаючи на цей URL JSON-дані про подію. |
| [Suppression](docs/Suppression.md) | `set`, `get`, `list`, `delete` | Методи для керування адресами, надсилання на які заблоковано з різних причин — відписка, недоступність, скарги. |
| [Domain](docs/Domain.md) | `get_dns_records`, `validate_verification_record`, `validate_dkim`, `list`, `get_sandbox_domain`, `delete` | Методи для налаштування доменів надсилання та перевірки їх статусів. Корисні, якщо вам недостатньо додавання доменів через вебінтерфейс AllyMailer і, наприклад, хочеться автоматизувати додавання доменів для ваших власних клієнтів. |
| [EventDump](docs/EventDump.md) | `create`, `get`, `list`, `delete` | Методи для отримання списку подій, що сталися, за певний період. Дані зберігаються до 45 днів, залежно від вашого тарифу. Це означає, що можна запросити події, починаючи з 00:00:00 45-го дня тому і до поточного моменту часу. Для індивідуальних тарифів з великими обсягами за замовчуванням події доступні лише за останні 7 днів. |
| [Tag](docs/Tag.md) | `list`, `delete` | Методи керування користувацькими мітками. За допомогою міток ви можете класифікувати листи за потрібними критеріями. Відокремити маркетингові листи від транзакційних або листи з підтвердженням реєстрації від сповіщень про замовлення. AllyMailer дозволяє позначати мітками кожне вихідне повідомлення, а потім сегментувати й аналізувати статистику на основі зазначених під час надсилання міток. |
| [Project](docs/Project.md) | `create`, `update`, `list`, `delete` | Проєкти дозволяють об'єднати кілька незалежних доменів із webhook, шаблонами та списками відписки. Це дуже корисно, якщо у вас є власні клієнти і ви керуєте їхніми даними з одного облікового запису AllyMailer. Кожен проєкт отримує власний ключ API, а виклики методів API з цим ключем обробляють лише дані цього проєкту. Таким чином, ви можете призначити окремий проєкт для кожного користувача і бути впевненими, що вони не конфліктуватимуть: наприклад, відписка від одного проєкту не вплине на інший. |
| [System](docs/System.md) | `ping`, `info` | Системні методи. |
| [Obsolete](docs/Obsolete.md) | `unsubscribed_set`, `unsubscribed_check`, `unsubscribed_list` | Застарілі методи роботи з відписками. Ми рекомендуємо користуватися замість них гнучкішими методами [suppression](https://docs.allymailer.com.ua/ua/web-api-ref#suppression). Зверніть увагу, що знову підписати відписаного адресата через WebAPI не можна. Можна лише надіслати йому email із посиланням повторної підписки за допомогою методу email/subscribe. |

### Власний HTTP-клієнт / заголовки

`ApiClient::with_http_client` приймає власний `reqwest::Client` (проксі, TLS, налаштування пулу), `UniOneClient::from_api_client` підключає його до фасаду; `Configuration::timeout` діє на кожен запит. Помилка транспорту зберігає свою `reqwest::Error` (`std::error::Error::source()`, `ApiError::transport_error()`, `is_timeout()`, `is_connect()`). У моделей із довільними ключами (`HeadersObject`) є словник `extra`: його ключі передаються поруч з іменованими полями.

```rust
use allymailer::common::model::HeadersObject;
use allymailer::{ApiClient, Configuration, AllyMailerClient};
use std::collections::HashMap;
use std::time::Duration;

let http = reqwest::Client::builder()
    .proxy(reqwest::Proxy::all("http://proxy.local:3128")?)
    .build()?;
let mut config = Configuration::new("YOUR_API_KEY");
config.timeout = Duration::from_secs(10);
let client = AllyMailerClient::from_api_client(ApiClient::with_http_client(config, http));

match client.system().ping().await {
    Err(e) if e.is_timeout() => eprintln!("timed out: {:?}", std::error::Error::source(&e)),
    other => println!("{:?}", other),
}

let headers = HeadersObject {
    extra: HashMap::from([
        ("X-MyHeader".to_string(), "some data".to_string()),
        ("CC".to_string(), "copy@example.com".to_string()),
        ("List-Unsubscribe".to_string(), "<mailto:unsubscribe@example.com>".to_string()),
    ]),
    ..Default::default()
};
```

### Обробка помилок

Кожен метод кидає `ApiError`, якщо API повернув помилку; у ньому є [код помилки API](https://docs.allymailer.com.ua/ua/web-api-ref#api-errors) і HTTP-статус.

## Допомога

- Для ШІ-агентів: [llms.txt](https://docs.allymailer.com.ua/llms.txt), скіли: `npx skills add https://github.com/allymailer/allymailer-skills.git`
- [Підтримка](https://cp.allymailer.com/ua/support) · [Задачі](https://github.com/allymailer/allymailer-rust-sdk/issues)
- Розробникам: [AGENTS.md](AGENTS.md) — SDK генерується з OpenAPI-специфікації.
