# Domain

[До списку розділів](../README.md)

Методи для налаштування доменів надсилання та перевірки їх статусів. Корисні, якщо вам недостатньо додавання доменів через вебінтерфейс AllyMailer і, наприклад, хочеться автоматизувати додавання доменів для ваших власних клієнтів.

Щоб додати новий домен надсилання через API, ви маєте зробити таке:
- Підтвердіть домен, з якого надсилатимуться листи, внісши необхідні DNS-записи, отримавши їх за допомогою методу [domain/get-dns-records](https://docs.allymailer.com.ua/ua/web-api-ref#domain-get-dns-records).
- Після додавання раз на 5–10 хвилин викликайте [domain/list](https://docs.allymailer.com.ua/ua/web-api-ref#domain-list) із зазначенням доданого домену (або кількох доменів одразу, якщо перевірити треба декілька) і перевіряйте verification-record.status та dkim.status у відповіді domain/list. Можна робити запит і частіше — але точно не частіше за раз на хвилину.
- Якщо verification-record.status="unconfirmed", то ініціюйте перевірку правильності запису володіння доменом за допомогою виклику [domain/validate-verification-record](https://docs.allymailer.com.ua/ua/web-api-ref#domain-validate-verification-record).
- Якщо dkim.status="inactive", то ініціюйте перевірку правильності DKIM-запису за допомогою виклику [domain/validate-dkim](https://docs.allymailer.com.ua/ua/web-api-ref#domain-validate-dkim) (втім, DKIM-записи AllyMailer і сам перевіряє раз на пів години).
- І лише коли domain/list повернув verification-record.status="confirmed" та dkim.status="active" для домену, можна використовувати домен для надсилання та припинити його очікування й виклики для нього зазначених вище методів.

`allymailer::api::domain::DomainApi`, `client.domain()`

| Метод | Ендпоінт | Опис |
|---|---|---|
| [`get_dns_records`](#get_dns_records) | `POST /domain/get-dns-records.json` | Повертає інформацію про записи, які потрібно внести в DNS для використання домену в AllyMailer. |
| [`validate_verification_record`](#validate_verification_record) | `POST /domain/validate-verification-record.json` | Ініціює перевірку запису про володіння доменом. |
| [`validate_dkim`](#validate_dkim) | `POST /domain/validate-dkim.json` | Перевіряє DKIM-запис домену. |
| [`list`](#list) | `POST /domain/list.json` | Повертає список усіх зареєстрованих доменів користувача або проєкту разом зі статусами останніх перевірок. |
| [`get_sandbox_domain`](#get_sandbox_domain) | `POST /domain/get-sandbox-domain.json` | Створює sandbox-домен облікового запису або повертає наявний. |
| [`delete`](#delete) | `POST /domain/delete.json` | Видаляє домен. Увага: для видалення домену, що належить проєкту, вам потрібно використати API-ключ проєкту. |

## get_dns_records

`POST /domain/get-dns-records.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#domain-get-dns-records)

Повертає інформацію про записи, які потрібно внести в DNS для використання домену в AllyMailer.

Щоб надсилати листи, вам потрібно підтвердити володіння доменом, зі зворотною адресою якого ви збираєтеся їх надсилати, а також налаштувати DKIM-підпис для цього домену. Метод дозволяє вам зареєструвати домен у AllyMailer і згенерувати запис для перевірки володіння та DKIM-ключ, або отримати їх, якщо вони були згенеровані раніше.

Після отримання успішної відповіді вам потрібно додати DNS-записи на кшталт цих:

`@             IN TXT "allymailer-validate-hash=483bb362ebdbeedd755cfb1d4d661"`

`us._domainkey IN TXT "v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDo7"`

`@             IN TXT  "v=spf1 include:spf.allymailer.com.ua ~all"`

`_dmarc        IN CNAME "example.com.dmarc.allymailer.com.ua."`

- Перший запис представлено вмістом поля "verification-record".
- Другий запис має включати вміст поля "dkim" одразу після "v=DKIM1; k=rsa; p=". Зверніть увагу на ім'я запису! Префікс "us._domainkey." дуже важливий для того, щоб DKIM-селектор AllyMailer працював правильно.
- Третій запис необов'язковий, але ми дуже рекомендуємо його додавати — це SPF-запис, який підвищує доставлюваність. Метод його не повертає. Можливо, вона у вас уже є — у цьому разі просто додайте include:spf.allymailer.com.ua після v=spf1.
- Четвертий запис — CNAME хоста "_dmarc" на значення поля "dmarc": тоді DMARC-політику домену публікує AllyMailer. Якщо в домену вже є свій TXT-запис DMARC, залиште його.

Вимоги до синтаксису записів у різних DNS-провайдерів можуть відрізнятися:
- іноді потрібно вказувати повне ім'я домену із завершальною крапкою, на кшталт "example.com." замість символа @.
- деякі провайдери вимагають указувати час життя (TTL, наприклад 3600)
- для деяких провайдерів буде зайвим поміщення значень у подвійні лапки
Будь ласка, перевірте вимоги свого провайдера й підкажіть це своїм користувачам.

Для реалізації white-label рішень є можливість змінити стандартні DNS-записи на ті, що не містять підрядка "allymailer". Щоб дізнатися докладніше — напишіть у техпідтримку.

### Запит: `DomainGetDnsRecordsRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `domain` | `String` | так | Домен, для якого потрібно отримати інформацію про DNS-записи. |

### Відповідь: `DomainGetDnsRecordsResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `domain` | `String` | Домен, для якого потрібно отримати інформацію про DNS-записи. |
| `verification_record` | `String` | Запис, який потрібно внести "як є" для підтвердження володіння цим доменом. |
| `dkim` | `String` | DKIM-підпис для домену. У цьому полі повертається лише ключ. Щоб TXT-запис був коректним, перед ключем потрібно додати "v=DKIM1; k=rsa; p=". |
| `dmarc` | `String` | Значення CNAME-запису для хоста "_dmarc" домену. |

### Приклад

```rust
use allymailer::api::domain::model::DomainGetDnsRecordsRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .get_dns_records(&DomainGetDnsRecordsRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.domain);
    Ok(())
}
```

## validate_verification_record

`POST /domain/validate-verification-record.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#domain-validate-verification-record)

Ініціює перевірку запису про володіння доменом.

Якщо результат виклику цього методу успішний (200 OK) — це означає, що перевірку запущено. Для отримання результату потрібно пізніше перевірити поле verification-record.status, що повертається методом [domain/list](https://docs.allymailer.com.ua/ua/web-api-ref#domain-list). Докладніше читайте [тут](https://docs.allymailer.com.ua/ua/web-api-ref#domain).

### Запит: `DomainValidateVerificationRecordRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `domain` | `String` | так | Домен, для якого перевіряється запис про володіння. |

### Відповідь: `DomainValidateVerificationRecordResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `message` | `String` | Налагоджувальне повідомлення про хід перевірки. |

### Приклад

```rust
use allymailer::api::domain::model::DomainValidateVerificationRecordRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .validate_verification_record(&DomainValidateVerificationRecordRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```

## validate_dkim

`POST /domain/validate-dkim.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#domain-validate-dkim)

Перевіряє DKIM-запис домену.

Якщо результат виклику цього методу успішний (200 OK) — це означає, що перевірку запущено. Для отримання результату потрібно пізніше перевірити поле dkim.status, що повертається методом [domain/list](https://docs.allymailer.com.ua/ua/web-api-ref#domain-list). Докладніше читайте [тут](https://docs.allymailer.com.ua/ua/web-api-ref#domain).

### Запит: `DomainValidateDkimRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `domain` | `String` | так | Домен, для якого перевіряється DKIM-запис. |

### Відповідь: `DomainValidateDkimResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `message` | `String` | Налагоджувальне повідомлення про хід перевірки. |

### Приклад

```rust
use allymailer::api::domain::model::DomainValidateDkimRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .validate_dkim(&DomainValidateDkimRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```

## list

`POST /domain/list.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#domain-list)

Повертає список усіх зареєстрованих доменів користувача або проєкту разом зі статусами останніх перевірок.

Повертає статус останньої перевірки DNS-запису володіння доменом і DKIM-запису для кожного домену. Докладніше [тут](https://docs.allymailer.com.ua/ua/web-api-ref#domain).

### Запит: `DomainListRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `domain` | `String` |  | Опціональне ім'я домену, якщо вам потрібно отримати статуси лише за одним доменом. |
| `limit` | `i64` |  | Максимальна кількість доменів, що повертаються. За замовчуванням 50. |
| `offset` | `i64` |  | Індекс першого домену зі списку, що повертається, рахуючи з нуля. |

### Відповідь: `DomainListResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `domains` | `Vec<DomainListResponseDomainsItem>` | Масив об'єктів, що описують домени. |

### Приклад

```rust
use allymailer::api::domain::model::DomainListRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .list(&DomainListRequest {
            domain: Some("example.com".to_string()),
            ..Default::default()
        })
        .await?;
    println!("{:?}", response.domains);
    Ok(())
}
```

## get_sandbox_domain

`POST /domain/get-sandbox-domain.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#domain-get-sandbox-domain)

Створює sandbox-домен облікового запису або повертає наявний.

Повертає sandbox-домен облікового запису, готовий до використання в полі `from_email` методу [email/send](https://docs.allymailer.com.ua/ua/web-api-ref#email-send). Якщо sandbox-домену немає, AllyMailer створює його та повертає. Повторні виклики повертають той самий домен.

Sandbox-домен видаляється через 31 день після створення; наступний виклик створить новий. З нього можна надіслати до 100 листів на день (ліміт облікового запису за замовчуванням).

Метод доступний лише з API-ключем облікового запису. API-ключі проєктів не можуть створювати sandbox-домени.

### Запит

Метод не має параметрів.

### Відповідь: `DomainGetSandboxDomainResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `domain` | `String` | Ім'я sandbox-домену. |

### Приклад

```rust
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client.domain().get_sandbox_domain().await?;
    println!("{:?}", response.domain);
    Ok(())
}
```

## delete

`POST /domain/delete.json` · [Документація](https://docs.allymailer.com.ua/ua/web-api-ref#domain-delete)

Видаляє домен. Увага: для видалення домену, що належить проєкту, вам потрібно використати API-ключ проєкту.

Видаляє домен.

### Запит: `DomainDeleteRequest`

| Поле | Тип | Обов'язкове | Опис |
|---|---|---|---|
| `domain` | `String` | так | Ім'я домену, який потрібно видалити. |

### Відповідь: `DomainDeleteResponse`

| Поле | Тип | Опис |
|---|---|---|
| `status` | `String` | Рядок "success". (`success`) |
| `message` | `String` | Домен видалено |

### Приклад

```rust
use allymailer::api::domain::model::DomainDeleteRequest;
use allymailer::AllyMailerClient;

#[tokio::main]
async fn main() -> Result<(), allymailer::ApiError> {
    let client = AllyMailerClient::new("YOUR_API_KEY");

    let response = client
        .domain()
        .delete(&DomainDeleteRequest {
            domain: "example.com".to_string(),
        })
        .await?;
    println!("{:?}", response.message);
    Ok(())
}
```
