# Email

[До списку розділів](../README.md)

Ця група методів пов'язана з надсиланням email.

`allymailer::api::email::EmailApi`, `client.email()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`send`](#send) | `POST /email/send.json` | Метод для надсилання листів вашим підписникам. |
| [`subscribe`](#subscribe) | `POST /email/subscribe.json` | Надіслати посилання на (пере)підписку. |

## send

`POST /email/send.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#email-send)

Метод для надсилання листів вашим підписникам.

Під час надсилання ви можете використовувати підстановки змінних, шаблони, увімкнути відстеження прочитань або переходів за посиланнями та багато іншого.

Обмеження:
* максимальний розмір запиту — 10 MB
* максимальна кількість отримувачів (recipients) — 500 email
* у назві вкладень заборонено символ '/'

Якщо під час надсилання листа у списку отримувачів будуть некоректно введені, неіснуючі, повторювані або відписані адреси, вони будуть повернуті у відповіді за допомогою параметра failed_emails, лист же буде надіслано на всі інші прийнятні адреси. Якщо всі адреси потраплять до failed_emails, надсилання здійснено не буде, буде повернуто помилку з API-кодом 204.

### Запит: `EmailSendRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `message` | `MessageObject` | так | Об'єкт, що містить усі властивості повідомлення, яке надсилається. |

### Відповідь: `EmailSendResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `job_id` | `String` | Ідентифікатор завдання з надсилання, може виявитися корисним під час з'ясування причин збоїв. |
| `emails` | `Vec<String>` | Масив email-адрес, успішно прийнятих до надсилання. |
| `failed_emails` | `HashMap<String, String>` | Об'єкт із email-адресами, не прийнятими до надсилання. Email представлено іменем властивості, а його статус — значенням властивості, наприклад: {"email1@gmail.com": "temporary_unavailable"}. Можливі статуси адрес, на які не вдалося здійснити надсилання:<br><br>- __unsubscribed__ — зазначену адресу відписано;<br>- __invalid__ — адреса не існує або введена некоректно;<br>- __duplicate__ — адреса повторюється в запиті, повторне надсилання на той самий email запобігнуто;<br>- __temporary_unavailable__ — адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:<br><br>- попереднє надсилання було відхилено сервером отримувача як спам;<br>- поштова скринька отримувача переповнена або не використовується;<br>- домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;<br>- сервер відправника було відхилено через блеклистинг;<br><br>- __permanent_unavailable__ — адреса перманентно недоступна через багаторазові недоставки;<br>- __complained__ — в одному з попередніх листів адресат натиснув “Це спам”;<br>- __blocked__ — надсилання на цю адресу заборонено адміністрацією AllyMailer.<br>У майбутньому можуть з'явитися й інші статуси. |

### Приклад

```rust
use allymailer::api::email::model::EmailSendRequest;
use allymailer::api::email::model::MessageObject;
use allymailer::api::email::model::MessageObjectRecipientsItem;
use allymailer::common::model::BodyObject;
use allymailer::common::model::TemplateEngine;
use allymailer::AllyMailerClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .email()
        .send(&EmailSendRequest {
            message: MessageObject {
                recipients: vec![
                    MessageObjectRecipientsItem {
                        email: "user@example.com".to_string(),
                        substitutions: Some(HashMap::from([
                            ("CustomerId".to_string(), "12452".to_string()),
                            ("to_name".to_string(), "John Smith".to_string()),
                        ])),
                        metadata: Some(HashMap::from([
                            ("campaign_id".to_string(), "c77f4f4e-3561-49f7-9f07-c35be01b4f43".to_string()),
                            ("customer_hash".to_string(), "b253ac7".to_string()),
                        ])),
                    },
                ],
                template_id: Some("00000000-0000-0000-0000-000000000000".to_string()),
                global_language: Some("en".to_string()),
                template_engine: Some(TemplateEngine::Velocity),
                body: BodyObject {
                    html: Some("<b>Hello, {{to_name}}</b>".to_string()),
                    plaintext: Some("Hello, {{to_name}}".to_string()),
                    amp: Some("<!doctype html><html amp4email><head> <meta charset=\"utf-8\"><script async src=\"https://cdn.ampproject.org/v0.js\"></script> <style amp4email-boilerplate>body{visibility:hidden}</style></head><body> Hello, AMP4EMAIL world.</body></html>".to_string()),
                },
                subject: "AllyMailer test email ".to_string(),
                from_email: Some("email@example.com".to_string()),
                from_name: Some("John Smith".to_string()),
                reply_to: Some("email@example.com".to_string()),
                reply_to_name: Some("John Smith".to_string()),
                idempotence_key: Some("SG1VsbG68sIH2dvc5mx890kIQ".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.job_id);
    Ok(())
}
```

## subscribe

`POST /email/subscribe.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#email-subscribe)

Надіслати посилання на (пере)підписку.

Метод дозволяє повторно підписати колишнього підписника, який відписався раніше, був заблокований через скарги або чия адреса недоступна з якоїсь іншої причини.

Під час виклику надсилається лист у нейтральному дизайні з посиланням підписки. При переході за посиланням з email підписник перестає бути недоступним для користувача та/або перестає бути глобально відписаним.

За замовчуванням дозволено лише одне надсилання листа повторної підписки на добу. Якщо вам потрібно більше — напишіть у техпідтримку.

### Запит: `EmailSubscribeRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `from_email` | `String` | так | Адреса відправника на підтвердженому домені. (Також підтримується іменування поля як "email_address_from"). |
| `from_name` | `String` | так | Ім'я відправника. (Також підтримується іменування поля як "name_from"). |
| `to_email` | `String` | так | Адреса отримувача. (Також підтримується іменування поля як "email_address_to"). |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::email::model::EmailSubscribeRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .email()
        .subscribe(&EmailSubscribeRequest {
            from_email: "email@example.com".to_string(),
            from_name: "Example".to_string(),
            to_email: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
