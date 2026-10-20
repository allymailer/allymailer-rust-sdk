# System

[До списку розділів](../README.md)

Системні методи.

`allymailer::api::system::SystemApi`, `client.system()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`ping`](#ping) | `POST /system/ping.json` | Перевіряє валідність API-ключа та повертає ідентифікатор користувача. |
| [`info`](#info) | `POST /system/info.json` | Повертає інформацію про користувача або проєкт за API-ключем. |

## ping

`POST /system/ping.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#system-ping)

Перевіряє валідність API-ключа та повертає ідентифікатор користувача.

### Запит

Метод не має параметрів.

### Відповідь: `SystemPingResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `user_id` | `i64` | Унікальний ідентифікатор користувача. |

### Приклад

```rust
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client.system().ping().await?;
    println!("{:?}", response.user_id);
    Ok(())
}
```

## info

`POST /system/info.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#system-info)

Повертає інформацію про користувача або проєкт за API-ключем.

### Запит

Метод не має параметрів.

### Відповідь: `SystemInfoResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `user_id` | `i64` | Унікальний ідентифікатор користувача. |
| `email` | `String` | Email користувача. |
| `project_id` | `String` | Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки. Присутній лише якщо API-ключ, використаний для запиту, був API-ключем проєкту. |
| `project_name` | `String` | Назва проєкту, унікальна в межах облікового запису користувача. Присутня лише якщо API-ключ, використаний для запиту, був API-ключем проєкту. |
| `project_accounting` | `ProjectAccountingObject` | Об'єкт із властивостями лічильника листів. Присутній лише якщо API-ключ, використаний для запиту, був API-ключем проєкту. |
| `accounting` | `SystemInfoResponseAccounting` | Об'єкт з властивостями періоду обліку. Присутній лише якщо API-ключ, використаний для запиту, був API-ключем користувача. |

### Приклад

```rust
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client.system().info().await?;
    println!("{:?}", response.user_id);
    Ok(())
}
```
