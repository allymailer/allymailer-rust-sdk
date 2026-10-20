# Obsolete

[До списку розділів](../README.md)

Застарілі методи роботи з відписками. Ми рекомендуємо користуватися замість них гнучкішими методами [suppression](https://docs.allymailer.com.ua/ua/web-api-ref#suppression). Зверніть увагу, що знову підписати відписаного адресата через WebAPI не можна. Можна лише надіслати йому email із посиланням повторної підписки за допомогою методу email/subscribe.

`allymailer::api::obsolete::ObsoleteApi`, `client.obsolete()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`unsubscribed_set`](#unsubscribed_set) | `POST /unsubscribed/set.json` | Внести email до списку відписаних. |
| [`unsubscribed_check`](#unsubscribed_check) | `POST /unsubscribed/check.json` | Перевіряє, чи відписано email. |
| [`unsubscribed_list`](#unsubscribed_list) | `POST /unsubscribed/list.json` | Повертає список адрес, відписаних починаючи з указаної дати. |

## unsubscribed_set

`POST /unsubscribed/set.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#unsubscribed-set)

Внести email до списку відписаних.

Після внесення email до списку відписаних усі майбутні спроби надіслати на нього повідомлення будуть відхилятися. Зауважте, що видалити email зі списку відписаних безпосередньо за допомогою API не можна. Але можна надіслати йому повідомлення з посиланням повторної підписки за допомогою методу [email/subscribe](https://docs.allymailer.com.ua/ua/web-api-ref#email-subscribe) або видалити його зі списку відписаних вручну на сторінці [Пошук email](https://cp.allymailer.com/ua/user/email) у панелі керування (але кількість таких видалень на день сильно обмежена).

### Запит: `ObsoleteUnsubscribedSetRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `address` | `String` | так | Email, який має бути відписано. |

### Відповідь: `ObsoleteUnsubscribedSetResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `address` | `String` | Відписаний email. |
| `message` | `ObsoleteUnsubscribedSetMessage` | Подробиці відписки:<br><br>- "unsubscribed" якщо email було щойно відписано,<br>- "was inactive" якщо він уже був позначений як неактивний і немає сенсу відписувати,<br>- "was unsubscribed" якщо email уже було відписано раніше,<br>- "added and unsubscribed" якщо для цього email не зберігалося жодного більш раннього статусу, тому його було додано й одразу ж відписано. (`unsubscribed`, `was inactive`, `was unsubscribed`, `added and unsubscribed`) |

### Приклад

```rust
use allymailer::api::obsolete::model::ObsoleteUnsubscribedSetRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .obsolete()
        .unsubscribed_set(&ObsoleteUnsubscribedSetRequest {
            address: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.address);
    Ok(())
}
```

## unsubscribed_check

`POST /unsubscribed/check.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#unsubscribed-check)

Перевіряє, чи відписано email.

### Запит: `ObsoleteUnsubscribedCheckRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `address` | `String` | так | Email, який необхідно перевірити. |

### Відповідь: `ObsoleteUnsubscribedCheckResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `address` | `String` | Email, який було перевірено. |
| `is_unsubscribed` | `bool` | true якщо відписано, false якщо ні. |

### Приклад

```rust
use allymailer::api::obsolete::model::ObsoleteUnsubscribedCheckRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .obsolete()
        .unsubscribed_check(&ObsoleteUnsubscribedCheckRequest {
            address: "email@example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.address);
    Ok(())
}
```

## unsubscribed_list

`POST /unsubscribed/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#unsubscribed-list)

Повертає список адрес, відписаних починаючи з указаної дати.

### Запит: `ObsoleteUnsubscribedListRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `date_from` | `String` |  | Дата у форматі РРРР-ММ-ДД, щоб отримати всі відписані адреси з "date_from" по сьогоднішній день. Якщо параметр не вказати, період за замовчуванням буде з 00:00 сьогоднішнього дня за UTC. |

### Відповідь: `ObsoleteUnsubscribedListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `unsubscribed` | `Vec<ObsoleteUnsubscribedListResponseUnsubscribedItem>` | Масив об'єктів з даними відписаних. |

### Приклад

```rust
use allymailer::api::obsolete::model::ObsoleteUnsubscribedListRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .obsolete()
        .unsubscribed_list(&ObsoleteUnsubscribedListRequest {
            date_from: Some("2020-10-14".to_string()),
        })
        .await?;
    println!("{:?}", response.unsubscribed);
    Ok(())
}
```
