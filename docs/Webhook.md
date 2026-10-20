# Webhook

[До списку розділів](../README.md)

AllyMailer дозволяє вам отримувати сповіщення про зміну статусу доставки листа або спам-блокування. Ви надаєте URL, і в разі виникнення події, що вас цікавить, ми сповіщаємо вас, передаючи на цей URL JSON-дані про подію.

Ви можете налаштувати формат виклику webhook (просто JSON або JSON, запакований у gzip), які саме події вас цікавлять, чи дозволено групувати кілька подій в одному виклику, чи потрібні вам такі подробиці, як SMTP-відповідь сервера або user agent отримувача тощо.

Зверніть увагу, що ваш обробник має вміти приймати щонайменше 5 паралельних викликів, до 100 подій у кожному (але ви можете налаштувати максимально дозволену кількість паралельних викликів, збільшивши її до 100).

Якщо URL, на який надсилається webhook, недоступний (немає відповіді HTTP 200 OK протягом 3 секунд), спроби надсилання webhook на цей URL до отримання очікуваного 200 OK триватимуть протягом 24 годин з інтервалом у 10 хвилин з додатковим параметром retry_count, значення якого збільшуватиметься на 1 з кожним повторним надсиланням webhook. Якщо протягом 24 годин сталося щонайменше 10 подій і жоден із багаторазових викликів webhook про ці події не був успішним, то ми вважаємо обробник непрацездатним і автоматично встановлюємо його статус у неактивний та повідомляємо про це на email, [вказаний в особистому кабінеті](https://cp.allymailer.com/ua/user/alerting/technical/).

Формат webhook і структура кожної події описані [тут](https://docs.allymailer.com.ua/ua/web-api-ref#callback-format).

`allymailer::api::webhook::WebhookApi`, `client.webhook()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`set`](#set) | `POST /webhook/set.json` | Встановлює або змінює властивості [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) — вашого обробника сповіщень про події. У момент виклику [webhook/set](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set) ми очікуємо, що до вказаного вами URL можна звернутися GET-запитом без параметрів і у відповідь надійде 200 OK. |
| [`get`](#get) | `POST /webhook/get.json` | Повертає властивості [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) |
| [`list`](#list) | `POST /webhook/list.json` | Повернути список усіх або кількох [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) (обробників сповіщень про події) користувача або проєкту. |
| [`delete`](#delete) | `POST /webhook/delete.json` | Видалити webhook. |

## set

`POST /webhook/set.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set)

Встановлює або змінює властивості [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) — вашого обробника сповіщень про події. У момент виклику [webhook/set](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set) ми очікуємо, що до вказаного вами URL можна звернутися GET-запитом без параметрів і у відповідь надійде 200 OK.

### Запит: `WebhookSetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `url` | `String` | так | Рядок з URL, на який надсилатиметься запит у разі виникнення події (ідентифікатор обробника). В URL поки що підтримуються лише ASCII-символи. Якщо вам потрібно використати не-ASCII, сконвертуйте URL у [Punycode](https://en.wikipedia.org/wiki/Punycode). |
| `status` | `WebhookStatus` |  | Статус webhook, за замовчуванням "active". Статус "disabled" означає, що webhook було вимкнено користувачем, "stopped" означає, що webhook було зупинено системою після 24 годин невдалих викликів (мінімум за 10 різними подіями). (`active`, `disabled`, `stopped`) |
| `event_format` | `WebhookEventFormat` |  | Формат сповіщень. "json_post"(за замовчуванням) або "json_post_gzip". (`json_post`, `json_post_gzip`) |
| `delivery_info` | `i64` |  | Повертати (1) чи ні (0) докладну інформацію про доставку. Якщо delivery_info=1, повертається додаткова інформація про відповідь сервера та внутрішній статус доставки в разі статусів доставки "hard_bounced" і "soft_bounced"; user agent, IP-адреса отримувача, геодані, визначена за user agent інформація про пристрій (тип і марка пристрою, назва ОС, браузера або поштового клієнта) і traffic_type (human, machine, protected або unknown) в разі статусів "opened" і "clicked"; URL для статусу "clicked". Для статусів "delivered", "soft_bounced" і "hard_bounced" AllyMailer також може повернути sender_ip — IP-адресу SMTP-сервера, з якого було надіслано лист, якщо він доступний. (`0`, `1`) |
| `single_event` | `i64` |  | 0=в одному виклику може надійти декілька подій, 1=лише одна подія на виклик (не рекомендується). (`0`, `1`) |
| `max_parallel` | `i64` |  | Максимальна кількість дозволених паралельних запитів до вашого сервера. Чим більше ваш сервер може обробити — тим краще. |
| `events` | `WebhookEventsObject` |  | Об'єкт, що описує події, про які потрібно сповіщати. |

### Відповідь: `WebhookSetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `object` | `WebhookObject` | Об'єкт з усіма властивостями webhook. |

### Приклад

```rust
use allymailer::api::webhook::model::WebhookSetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .webhook()
        .set(&WebhookSetRequest {
            url: "https://yourhost.example.com/allymailer-webhook".to_string(),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.object);
    Ok(())
}
```

## get

`POST /webhook/get.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-get)

Повертає властивості [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook)

### Запит: `WebhookGetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `url` | `String` | так | URL webhook. |

### Відповідь: `WebhookGetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `object` | `WebhookObject` | Об'єкт з усіма властивостями webhook. |

### Приклад

```rust
use allymailer::api::webhook::model::WebhookGetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .webhook()
        .get(&WebhookGetRequest {
            url: "https://example.com/webhook".to_string(),
        })
        .await?;
    println!("{:?}", response.object);
    Ok(())
}
```

## list

`POST /webhook/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-list)

Повернути список усіх або кількох [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) (обробників сповіщень про події) користувача або проєкту.

### Запит: `WebhookListRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `limit` | `i64` |  | Максимальна кількість webhook, що повертаються за один виклик. Рекомендується 50. |
| `offset` | `i64` |  | Індекс першого webhook зі списку, що повертається, рахуючи з нуля. |

### Відповідь: `WebhookListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `objects` | `Vec<WebhookObject>` | Масив об'єктів з даними про webhook. |

### Приклад

```rust
use allymailer::api::webhook::model::WebhookListRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .webhook()
        .list(&WebhookListRequest {
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.objects);
    Ok(())
}
```

## delete

`POST /webhook/delete.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-delete)

Видалити webhook.

Ви також можете тимчасово вимкнути webhook без його видалення, встановивши його поле status у "disabled" за допомогою методу [webhook/set](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set).

### Запит: `WebhookDeleteRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `url` | `String` | так | URL webhook. |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::webhook::model::WebhookDeleteRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .webhook()
        .delete(&WebhookDeleteRequest {
            url: "https://example.com/webhook".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```

## Колбеки: `WebhookCallback`

Опис формату webhook.

Розбір тіла вхідного запиту вебхука:

```rust
use allymailer::api::webhook::WebhookApi;

let callback = WebhookApi::parse_callback(&request_body)?;
println!("{:?}", callback);
```

| Поле | Тип | Опис |
|---|---|---|
| `auth` | `String` | MD5-хеш рядкового тіла повідомлення, у якому значення auth замінено на api_key користувача/проєкту, чий обробник викликається. За допомогою цього отримувач сповіщення може як провести автентифікацію, так і перевірити цілісність сповіщення. |
| `events_by_user` | `Vec<WebhookCallbackEventsByUserItem>` | Масив з одним елементом, що містить дані про події користувача/проєкту. |
