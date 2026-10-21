# Template

[До списку розділів](../README.md)

Ви можете зберегти шаблон на боці AllyMailer і потім посилатися на нього за ідентифікатором. Це не обов'язково, але вкрай рекомендовано у разі повторюваних email-повідомлень або за великих обсягів надсилання. AllyMailer підтримує два різні синтаксиси для шаблонів, докладніше читайте [тут](https://docs.allymailer.com.ua/ua/template-engines).

`allymailer::api::template::TemplateApi`, `client.template()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`set`](#set) | `POST /template/set.json` | Створити або оновити шаблон. |
| [`get`](#get) | `POST /template/get.json` | Повертає властивості шаблону за його ідентифікатором. |
| [`list`](#list) | `POST /template/list.json` | Повертає повний список або частину списку всіх шаблонів. |
| [`delete`](#delete) | `POST /template/delete.json` | Видаляє шаблон за ідентифікатором. |

## set

`POST /template/set.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#template-set)

Створити або оновити шаблон.

Обмеження:
* Максимальний розмір запиту — 10 MB.
* Кількість шаблонів обмежено 10000.

### Запит: `TemplateSetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `template` | `TemplateObject` | так | Об'єкт, що містить властивості шаблону. |

### Відповідь: `TemplateSetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `template` | `TemplateObject` | Об'єкт, що містить властивості шаблону. |

### Приклад

```rust
use allymailer::api::template::model::EditorType;
use allymailer::api::template::model::TemplateObject;
use allymailer::api::template::model::TemplateSetRequest;
use allymailer::common::model::TemplateEngine;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .set(&TemplateSetRequest {
            template: TemplateObject {
                name: "name".to_string(),
                editor_type: Some(EditorType::Html),
                template_engine: Some(TemplateEngine::Velocity),
                subject: Some("AllyMailer test email ".to_string()),
                from_email: "user@example.com".to_string(),
                from_name: Some("John Smith".to_string()),
                reply_to: Some("email@example.com".to_string()),
                reply_to_name: Some("John Smith".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("{:?}", response.template);
    Ok(())
}
```

## get

`POST /template/get.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#template-get)

Повертає властивості шаблону за його ідентифікатором.

### Запит: `TemplateGetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `id` | `String` | так | Ідентифікатор шаблону. |

### Відповідь: `TemplateGetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `template` | `TemplateObject` | Об'єкт, що містить властивості шаблону. |

### Приклад

```rust
use allymailer::api::template::model::TemplateGetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .get(&TemplateGetRequest {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
        })
        .await?;
    println!("{:?}", response.template);
    Ok(())
}
```

## list

`POST /template/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#template-list)

Повертає повний список або частину списку всіх шаблонів.

Ви можете переглядати великий список частинами, використовуючи параметри "offset" і "limit". Якщо повернуто менше ніж "limit" шаблонів — значить, ви досягли кінця списку.

### Запит: `TemplateListRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `limit` | `i64` |  | Максимальна кількість шаблонів, що повертаються, за замовчуванням 50. |
| `offset` | `i64` |  | Індекс першого шаблону зі списку, що повертається, рахуючи з нуля. |

### Відповідь: `TemplateListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `templates` | `Vec<TemplateListObject>` | Масив об'єктів-шаблонів. |

### Приклад

```rust
use allymailer::api::template::model::TemplateListRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .list(&TemplateListRequest {
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.templates);
    Ok(())
}
```

## delete

`POST /template/delete.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#template-delete)

Видаляє шаблон за ідентифікатором.

### Запит: `TemplateDeleteRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `id` | `String` | так | Ідентифікатор шаблону. |

### Відповідь: `SuccessResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |

### Приклад

```rust
use allymailer::api::template::model::TemplateDeleteRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .template()
        .delete(&TemplateDeleteRequest {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
        })
        .await?;
    println!("{:?}", response);
    Ok(())
}
```
