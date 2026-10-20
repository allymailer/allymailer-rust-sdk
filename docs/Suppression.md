# Suppression

[До списку розділів](../README.md)

Методи для керування адресами, надсилання на які заблоковано з різних причин — відписка, недоступність, скарги.

`allymailer::api::suppression::SuppressionApi`, `client.suppression()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`set`](#set) | `POST /suppression/set.json` | Додає email-адресу до списку недоступних. Ви завжди зможете пізніше видалити цю адресу зі списку недоступних методом [suppression/delete](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-delete). |
| [`get`](#get) | `POST /suppression/get.json` | Повертає причину й дату блокування email-адреси. |
| [`list`](#list) | `POST /suppression/list.json` | Повертає список недоступних, починаючи з указаної дати. |
| [`delete`](#delete) | `POST /suppression/delete.json` | Видаляє email-адресу зі списку недоступних (лише записи з is_deletable=true). Якщо жодного запису для email не знайдено, повертається помилка API 3003. Якщо записи про недоступність знайдено, але жодну з них видалити не можна, повертається помилка API 3004. Якщо перевищено ліміт видалень на добу, повертається помилка API 906. |

## set

`POST /suppression/set.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-set)

Додає email-адресу до списку недоступних. Ви завжди зможете пізніше видалити цю адресу зі списку недоступних методом [suppression/delete](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-delete).

### Запит: `SuppressionSetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `email` | `String` | так | Email-адреса, яку потрібно додати до списку недоступних. |
| `cause` | `String` | так | Причина, з якої надсилання на цей email блокується. Одна з таких:<br><br>- __unsubscribed__ - зазначену адресу відписано;<br>- __temporary_unavailable__ - адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:<br><br>- попереднє надсилання було відхилено сервером отримувача як спам;<br>- поштова скринька отримувача переповнена або не використовується;<br>- домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;<br>- сервер відправника було відхилено через блеклистинг;<br><br>- __permanent_unavailable__ - адреса перманентно недоступна через багаторазові недоставки;<br>- __complained__ - в одному з попередніх листів адресат натиснув “Це спам”. |
| `created` | `String` |  | Коли було встановлено недоступність, у часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс". |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::suppression::model::SuppressionSetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .set(&SuppressionSetRequest {
            email: "email@example.com".to_string(),
            cause: "unsubscribed".to_string(),
            created: Some("2021-12-19 10:15:49".to_string()),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```

## get

`POST /suppression/get.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-get)

Повертає причину й дату блокування email-адреси.

### Запит: `SuppressionGetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `email` | `String` | так | Email-адреса, для якої потрібно отримати дані про недоступність. |
| `all_projects` | `bool` |  | Якщо у користувача увімкнено проєкти, all_projects=true дозволить шукати за даними всіх проєктів. |

### Відповідь: `SuppressionGetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `email` | `String` | Email-адреса, для якої було запитано дані про недоступність. |
| `suppressions` | `Vec<SuppressionGetObject>` | Масив об'єктів з інформацією про недоступність. |

### Приклад

```rust
use allymailer::api::suppression::model::SuppressionGetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .get(&SuppressionGetRequest {
            email: "email@example.com".to_string(),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.email);
    Ok(())
}
```

## list

`POST /suppression/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-list)

Повертає список недоступних, починаючи з указаної дати.

### Запит: `SuppressionListRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `cause` | `String` |  | Причина, з якої надсилання на цей email блокується. Одна з таких:<br><br>- __unsubscribed__ - зазначену адресу відписано;<br>- __temporary_unavailable__ - адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:<br><br>- попереднє надсилання було відхилено сервером отримувача як спам;<br>- поштова скринька отримувача переповнена або не використовується;<br>- домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;<br>- сервер відправника було відхилено через блеклистинг;<br><br>- __permanent_unavailable__ - адреса перманентно недоступна через багаторазові недоставки;<br>- __complained__ - в одному з попередніх листів адресат натиснув “Це спам”.<br>- __blocked__ - надсилання на цю адресу заборонено адміністрацією AllyMailer.<br>У майбутньому можуть з'явитися й інші причини. |
| `source` | `String` |  | Джерело потрапляння до списку недоступних. Одне з таких:<br><br>- __user__ - дії користувача за допомогою методу [suppression/set](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-set);<br>- __system__ - надсилання на цю адресу заборонено системою, наприклад через багаторазові недоставки;<br>- __subscriber__ - у зв'язку з діями отримувача. В одному з попередніх листів адресат натиснув “Це спам” або відписався. |
| `start_time` | `String` |  | Дата у форматі РРРР-ММ-ДД гг:хх:сс, щоб отримати список блокування з "start_time" по сьогоднішній день. Ігнорується, якщо параметр “cursor” не порожній. |
| `cursor` | `String` |  | Параметр указує, з якої позиції починати вибірку. Має бути порожнім або відсутнім для отримання першої порції даних; для отримання другої та наступних порцій його необхідно задавати, підставляючи значення поля "cursor" з попередньої порції. |
| `limit` | `i64` |  | Максимум записів, що видаються за один раз, за замовчуванням 50. |

### Відповідь: `SuppressionListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `suppressions` | `Vec<SuppressionListObject>` | Масив об'єктів з інформацією про недоступність. |
| `cursor` | `String` | Параметр указує, з якої позиції починати вибірку. Має бути порожнім або відсутнім для отримання першої порції даних; для отримання другої та наступних порцій його необхідно задавати, підставляючи значення поля "cursor" з попередньої порції. |

### Приклад

```rust
use allymailer::api::suppression::model::SuppressionListRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .list(&SuppressionListRequest {
            cause: Some("unsubscribed".to_string()),
            source: Some("user".to_string()),
            start_time: Some("2020-10-14 17:00:00".to_string()),
            cursor: Some("Ajfb6Hvdkn3hdhhvG57xbdufhG5".to_string()),
            limit: Some(50),
        })
        .await?;
    println!("{:?}", response.suppressions);
    Ok(())
}
```

## delete

`POST /suppression/delete.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-delete)

Видаляє email-адресу зі списку недоступних (лише записи з is_deletable=true). Якщо жодного запису для email не знайдено, повертається помилка API 3003. Якщо записи про недоступність знайдено, але жодну з них видалити не можна, повертається помилка API 3004. Якщо перевищено ліміт видалень на добу, повертається помилка API 906.

### Запит: `SuppressionDeleteRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `email` | `String` | так | Email-адреса, яку потрібно видалити зі списку недоступних. |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::suppression::model::SuppressionDeleteRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .suppression()
        .delete(&SuppressionDeleteRequest {
            email: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
