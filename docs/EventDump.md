# EventDump

[До списку розділів](../README.md)

Методи для отримання списку подій, що сталися, за певний період. Дані зберігаються до 45 днів, залежно від вашого тарифу. Це означає, що можна запросити події, починаючи з 00:00:00 45-го дня тому і до поточного моменту часу. Для індивідуальних тарифів з великими обсягами за замовчуванням події доступні лише за останні 7 днів.

`allymailer::api::event_dump::EventDumpApi`, `client.event_dump()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`create`](#create) | `POST /event-dump/create.json` | Асинхронний метод, який ініціює підготовку для завантаження файлів дампа у форматі CSV. Максимальна кількість збережених/створюваних дампів обмежена десятьма. У разі досягнення ліміту для запиту на підготовку нового дампа потрібно видалити старі. Час зберігання сформованого дампа — 8 годин, після чого він автоматично видаляється. Якщо період запиту даних більший за добу або подій багато, буде сформовано декілька файлів, у кожному з яких буде не більше ніж 100 000 подій. Файл містить колонки з такою інформацією — 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags' і 'project_id' (якщо у користувача увімкнено проєкти). Ви можете відфільтрувати поля та змінити їх порядок, а також додати поле 'subject' за допомогою параметра 'dump_fileds'. |
| [`get`](#get) | `POST /event-dump/get.json` | Повертає властивості дампа за його ідентифікатором. |
| [`list`](#list) | `POST /event-dump/list.json` | Метод без параметрів, повертає список усіх дампів подій. |
| [`delete`](#delete) | `POST /event-dump/delete.json` | Видаляє дамп із черги та/або зі сховища. |

## create

`POST /event-dump/create.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create)

Асинхронний метод, який ініціює підготовку для завантаження файлів дампа у форматі CSV. Максимальна кількість збережених/створюваних дампів обмежена десятьма. У разі досягнення ліміту для запиту на підготовку нового дампа потрібно видалити старі. Час зберігання сформованого дампа — 8 годин, після чого він автоматично видаляється. Якщо період запиту даних більший за добу або подій багато, буде сформовано декілька файлів, у кожному з яких буде не більше ніж 100 000 подій. Файл містить колонки з такою інформацією — 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags' і 'project_id' (якщо у користувача увімкнено проєкти). Ви можете відфільтрувати поля та змінити їх порядок, а також додати поле 'subject' за допомогою параметра 'dump_fileds'.

### Запит: `EventDumpCreateRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `start_time` | `String` | так | Дата й час у форматі РРРР-ММ-ДД гг:хх:сс, починаючи з якої необхідно отримати події. Дані зберігаються до 45 днів, залежно від вашого тарифу. |
| `end_time` | `String` |  | Дата й час у форматі РРРР-ММ-ДД гг:хх:сс, до якої (не включно) запитуються події. |
| `limit` | `i64` |  | Максимальна кількість подій, що повертаються, за замовчуванням 50. Якщо це значення перевищує 100 000, буде створено декілька файлів, кожен з яких містить максимум 100 000 подій. |
| `all_projects` | `bool` |  | Якщо у користувача увімкнено проєкти, all_projects=true дозволить отримати дані всіх проєктів. |
| `filter` | `FilterObject` |  | Об'єкт із властивостями фільтра дампа подій. |
| `dump_fields` | `Vec<String>` |  | Масив із рядків з переліком назв стовпців, які необхідно отримати у вивантаженні (у порядку переліку). Якщо 'dump_fields' відсутній або порожній — застосовується набір і порядок полів за замовчуванням. Припустимі значення (у порядку за замовчуванням): 'project_id', 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags'. З метою більшої конфіденційності поле 'subject' не входить до набору полів за замовчуванням, але ви можете додати його, вказавши в параметрі 'dump_fields'. |
| `aggregate` | `String` |  | Дозволяє отримати агреговану статистику за вказаний період. Може набувати одного з двох значень:<br><br>- __day_status__ - отримати агреговану статистику; за такого значення параметр filter має бути відсутнім або порожнім об'єктом (фільтри неприпустимі);<br>- __порожній рядок__ - значення за замовчуванням, еквівалентне відсутності параметра aggregate, і метод event_dump/create у цьому разі працює як раніше. |
| `delimiter` | `String` |  | Роздільник полів, за замовчуванням ','. Припустимі значення ',' і ';'. |
| `format` | `String` |  | Формат отримуваного файлу “csv” (за замовчуванням) або “csv_gzip”. |

### Відповідь: `EventDumpCreateResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `dump_id` | `String` | Ідентифікатор дампа. Використовується для виклику методу [event-dump/get](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-get). |

### Приклад

```rust
use allymailer::api::event_dump::model::EventDumpCreateRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .event_dump()
        .create(&EventDumpCreateRequest {
            start_time: "2022-07-20 00:00:00".to_string(),
            end_time: Some("2022-07-21 00:00:00".to_string()),
            limit: Some(50),
            dump_fields: Some(vec!["string1".to_string()]),
            aggregate: Some("day_status".to_string()),
            delimiter: Some(";".to_string()),
            format: Some("csv".to_string()),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.dump_id);
    Ok(())
}
```

## get

`POST /event-dump/get.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-get)

Повертає властивості дампа за його ідентифікатором.

### Запит: `EventDumpGetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `dump_id` | `String` | так | Ідентифікатор дампа, отриманий методом [event-dump/create](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create). |

### Відповідь: `EventDumpGetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `event_dump` | `EventDumpObject` | Об'єкт із властивостями дампа подій. |

### Приклад

```rust
use allymailer::api::event_dump::model::EventDumpGetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .event_dump()
        .get(&EventDumpGetRequest {
            dump_id: "Gqfasjh34tlasd".to_string(),
        })
        .await?;
    println!("{:?}", response.event_dump);
    Ok(())
}
```

## list

`POST /event-dump/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-list)

Метод без параметрів, повертає список усіх дампів подій.

### Запит

Метод не має параметрів.

### Відповідь: `EventDumpListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `event_dumps` | `Vec<EventDumpObject>` | Масив об'єктів, що описують дампи подій. Кожен об'єкт містить ті самі властивості, що й об'єкт 'event_dump', який повертає метод [event-dump/get](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-get). |

### Приклад

```rust
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client.event_dump().list().await?;
    println!("{:?}", response.event_dumps);
    Ok(())
}
```

## delete

`POST /event-dump/delete.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-delete)

Видаляє дамп із черги та/або зі сховища.

### Запит: `EventDumpDeleteRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `dump_id` | `String` | так | Ідентифікатор дампа, отриманий методом [event-dump/create](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create). |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::event_dump::model::EventDumpDeleteRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .event_dump()
        .delete(&EventDumpDeleteRequest {
            dump_id: "Gqfasjh34tlasd".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
