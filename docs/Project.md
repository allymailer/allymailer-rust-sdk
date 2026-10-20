# Project

[До списку розділів](../README.md)

Проєкти дозволяють об'єднати кілька незалежних доменів із webhook, шаблонами та списками відписки. Це дуже корисно, якщо у вас є власні клієнти і ви керуєте їхніми даними з одного облікового запису AllyMailer. Кожен проєкт отримує власний ключ API, а виклики методів API з цим ключем обробляють лише дані цього проєкту. Таким чином, ви можете призначити окремий проєкт для кожного користувача і бути впевненими, що вони не конфліктуватимуть: наприклад, відписка від одного проєкту не вплине на інший.

Методи проєктів за замовчуванням вимкнено через заходи захисту від спаму. Щоб їх увімкнути, вам необхідно звернутися до [служби підтримки](https://cp.allymailer.com/ua/support).

Зверніть увагу: за замовчуванням домени для надсилання підтверджуються та використовуються незалежно для основного облікового запису й кожного з проєктів. Але є можливість увімкнути для проєктів використання доменів, підтверджених в основному обліковому записі. Просто попросіть про це [техпідтримку](https://cp.allymailer.com/ua/support).

`allymailer::api::project::ProjectApi`, `client.project()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`create`](#create) | `POST /project/create.json` | Створити новий проєкт. |
| [`update`](#update) | `POST /project/update.json` | Змінити властивості проєкту. |
| [`list`](#list) | `POST /project/list.json` | Повертає список проєктів. |
| [`delete`](#delete) | `POST /project/delete.json` | Видалити проєкт. |

## create

`POST /project/create.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#project-create)

Створити новий проєкт.

### Запит: `ProjectCreateRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `project` | `ProjectObject` | так | Об'єкт із властивостями проєкту. |

### Відповідь: `ProjectCreateResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `project_id` | `String` | Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки. |
| `project_api_key` | `String` | API-ключ проєкту. Ви можете використовувати цей ключ замість користувацького api_key у всіх методах, крім project-методів. |

### Приклад

```rust
use allymailer::api::project::model::ProjectCreateRequest;
use allymailer::api::project::model::ProjectObject;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .create(&ProjectCreateRequest {
            project: ProjectObject {
                name: "Project 1A".to_string(),
                country: Some("ES".to_string()),
                backend_domain_id: Some(1234),
                unsubscribe_page_id: Some(1),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.project_id);
    Ok(())
}
```

## update

`POST /project/update.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#project-update)

Змінити властивості проєкту.

### Запит: `ProjectUpdateRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `project_id` | `String` |  | Ідентифікатор проєкту, що оновлюється. Проєкт має бути ідентифіковано або цим полем project_id, або полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше. |
| `project_api_key` | `String` |  | Необов'язковий API-ключ проєкту, що оновлюється. Проєкт має бути ідентифіковано або полем project_id, або полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше. |
| `project` | `ProjectUpdateObject` | так | Об'єкт із властивостями проєкту. |

### Відповідь: `ProjectUpdateResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `project_api_key` | `String` | API-ключ проєкту. Повертається лише в разі, якщо в параметрі запиту був project_api_key. Якщо ж проєкт було ідентифіковано через project_id, то не повертається. |

### Приклад

```rust
use allymailer::api::project::model::ProjectUpdateObject;
use allymailer::api::project::model::ProjectUpdateRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .update(&ProjectUpdateRequest {
            project_id: Some("1234567890".to_string()),
            project_api_key: Some("your api_key".to_string()),
            project: ProjectUpdateObject {
                name: "Project 1A".to_string(),
                country: Some("ES".to_string()),
                backend_domain_id: Some(1234),
                unsubscribe_page_id: Some(1),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.project_api_key);
    Ok(())
}
```

## list

`POST /project/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#project-list)

Повертає список проєктів.

### Запит: `ProjectListRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `project_id` | `String` |  | Опціональна можливість указати project_id для включення до списку лише вказаного проєкту. |
| `project_api_key` | `String` |  | Опціональна можливість указати project_api_key для включення до списку лише вказаного проєкту. |

### Відповідь: `ProjectListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `projects` | `Vec<ProjectListObject>` | Масив об'єктів, де кожен об'єкт відповідає окремому проєкту. |

### Приклад

```rust
use allymailer::api::project::model::ProjectListRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .list(&ProjectListRequest {
            project_id: Some("1234567890".to_string()),
            project_api_key: Some("your api_key".to_string()),
        })
        .await?;
    println!("{:?}", response.projects);
    Ok(())
}
```

## delete

`POST /project/delete.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#project-delete)

Видалити проєкт.

Деякий час проєкт не видаляється остаточно, і його ще можна відновити — для цього необхідно звернутися в техпідтримку.

### Запит: `ProjectDeleteRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `project_id` | `String` |  | Ідентифікатор проєкту, що видаляється. Проєкт має бути ідентифіковано або цим полем project_id, або полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше. |
| `project_api_key` | `String` |  | Необов'язковий API-ключ проєкту, що видаляється. Проєкт має бути ідентифіковано або полем project_id, або полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше. |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::project::model::ProjectDeleteRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .project()
        .delete(&ProjectDeleteRequest {
            project_id: Some("1234567890".to_string()),
            project_api_key: Some("your api_key".to_string()),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
