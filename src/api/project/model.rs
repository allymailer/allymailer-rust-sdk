//! Models of the Project section.

use serde::{Deserialize, Serialize};

/// ProjectCreateRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateRequest {
    /// Об'єкт із властивостями проєкту.
    #[serde(rename = "project", default)]
    pub project: ProjectObject,
}

/// Об'єкт із властивостями проєкту.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectObject {
    /// Назва проєкту, унікальна в межах облікового запису користувача.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Двобуквений код країни за правилами ISO-3166 alpha-2. Якщо задано, то AllyMailer обробляє
    /// персональні дані проєкту відповідно до законів заданої країни — наприклад, відповідно до GDPR для
    /// європейських країн.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Чи дозволено надсилання листів для проєкту.
    /// За замовчуванням: `True`.
    #[serde(
        rename = "send_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_enabled: Option<bool>,
    /// Якщо false, то AllyMailer додає в кінець кожного листа системне посилання відписки. Значення true
    /// вимикає автоматичне додавання системою посилання відписки в кінець листа й дозволяє використовувати
    /// власне посилання відписки або надсилати листи зовсім без посилання відписки. Докладніше див.
    /// [тут](https://docs.allymailer.com.ua/ua/unsubscribe-link). Якщо ви не вказуєте значення
    /// custom_unsubscribe_url_enabled під час створення проєкту — то це значення копіюється з головного
    /// облікового запису (якщо в головному обліковому записі було встановлено примусове вимкнення посилання
    /// відписки — то й у проєкту посилання відписки буде вимкнено).
    #[serde(
        rename = "custom_unsubscribe_url_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_unsubscribe_url_enabled: Option<bool>,
    /// Унікальний ідентифікатор backend-домену, який визначає [домен посилань](/tracking-domains) або пул
    /// [виділених IP-адрес](/dedicated-ip) за замовчуванням. Якщо відсутній — обліковому запису/проєкту
    /// буде призначено дефолтний backend-домен за замовчуванням.
    #[serde(
        rename = "backend_domain_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_domain_id: Option<i64>,
    /// Унікальний ідентифікатор, який визначає сторінку відписки за замовчуванням. Якщо відсутній —
    /// обліковому запису/проєкту буде призначено системну сторінку відписки за замовчуванням.
    #[serde(
        rename = "unsubscribe_page_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_page_id: Option<i64>,
}

/// ProjectCreateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// API-ключ проєкту. Ви можете використовувати цей ключ замість користувацького api_key у всіх методах,
    /// крім project-методів.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}

/// ProjectUpdateRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateRequest {
    /// Ідентифікатор проєкту, що оновлюється. Проєкт має бути ідентифіковано або цим полем project_id, або
    /// полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Необов'язковий API-ключ проєкту, що оновлюється. Проєкт має бути ідентифіковано або полем
    /// project_id, або полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
    /// Об'єкт із властивостями проєкту.
    #[serde(rename = "project", default)]
    pub project: ProjectUpdateObject,
}

/// Об'єкт із властивостями проєкту.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateObject {
    /// Назва проєкту, унікальна в межах облікового запису користувача.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Двобуквений код країни за правилами ISO-3166 alpha-2. Якщо задано, то AllyMailer обробляє
    /// персональні дані проєкту відповідно до законів заданої країни — наприклад, відповідно до GDPR для
    /// європейських країн.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Чи дозволено надсилання листів для проєкту.
    /// За замовчуванням: `True`.
    #[serde(
        rename = "send_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_enabled: Option<bool>,
    /// Якщо false, то AllyMailer додає в кінець кожного листа системне посилання відписки. Значення true
    /// вимикає автоматичне додавання системою посилання відписки в кінець листа й дозволяє використовувати
    /// власне посилання відписки або надсилати листи зовсім без посилання відписки. Докладніше див.
    /// [тут](https://docs.allymailer.com.ua/ua/unsubscribe-link). Якщо ви не вказуєте значення
    /// custom_unsubscribe_url_enabled під час створення проєкту — то це значення копіюється з головного
    /// облікового запису (якщо в головному обліковому записі було встановлено примусове вимкнення посилання
    /// відписки — то й у проєкту посилання відписки буде вимкнено).
    #[serde(
        rename = "custom_unsubscribe_url_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_unsubscribe_url_enabled: Option<bool>,
    /// Унікальний ідентифікатор backend-домену, який визначає [домен посилань](/tracking-domains) або пул
    /// [виділених IP-адрес](/dedicated-ip) за замовчуванням. Якщо відсутній — обліковому запису/проєкту
    /// буде призначено дефолтний backend-домен за замовчуванням.
    #[serde(
        rename = "backend_domain_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_domain_id: Option<i64>,
    /// Унікальний ідентифікатор, який визначає сторінку відписки за замовчуванням. Якщо відсутній —
    /// обліковому запису/проєкту буде призначено системну сторінку відписки за замовчуванням.
    #[serde(
        rename = "unsubscribe_page_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_page_id: Option<i64>,
    /// Значення лічильника листів для проєкту, збільшується під час надсилання кожного листа. Приймає лише
    /// значення 0 (тобто скидання лічильника в нуль) або оновлюється автоматично, залежно від режиму роботи
    /// лічильника, заданого параметром 'email_counter_mode'. Якщо поле не передано в запиті або передано
    /// null, то значення лічильника не змінюється.
    #[serde(
        rename = "email_counter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter: Option<i64>,
    /// Дозволяє обмежити кількість надісланих проєктом листів. У разі досягнення цього ліміту надсилання
    /// припиняється. Приймається будь-яке невід'ємне 64-бітне ціле число. Значення 0 означає відсутність
    /// ліміту. Якщо поле не передано в запиті або передано null, то значення ліміту не змінюється.
    /// За замовчуванням: `0`.
    #[serde(
        rename = "email_counter_limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_limit: Option<i64>,
    /// Встановлює режим роботи лічильника листів. Дозволяє вибрати, як має поводитися лічильник:
    /// автоматично скидатися чи залишатися постійним. Припустимі значення: default - лічильник автоматично
    /// скидається в нуль на початку кожного облікового періоду, permanent - лічильник ніколи не скидається
    /// автоматично. Скидання може бути виконане лише вручну користувачем, передавши значення 0 у полі
    /// 'email_counter'. Якщо поле не передано в запиті або передано значення null, значення режиму не
    /// змінюється.
    /// За замовчуванням: `default`.
    #[serde(
        rename = "email_counter_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_mode: Option<crate::common::model::EmailCounterMode>,
}

/// ProjectUpdateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// API-ключ проєкту. Повертається лише в разі, якщо в параметрі запиту був project_api_key. Якщо ж
    /// проєкт було ідентифіковано через project_id, то не повертається.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}

/// ProjectListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectListRequest {
    /// Опціональна можливість указати project_id для включення до списку лише вказаного проєкту.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Опціональна можливість указати project_api_key для включення до списку лише вказаного проєкту.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}

/// ProjectListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів, де кожен об'єкт відповідає окремому проєкту.
    #[serde(rename = "projects", default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectListObject>>,
}

/// ProjectListObject
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectListObject {
    /// Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// API-ключ проєкту. Ви можете використовувати цей ключ замість користувацького api_key у всіх методах,
    /// крім project-методів.
    #[serde(rename = "api_key", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Назва проєкту, унікальна в межах облікового запису користувача.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Двобуквений код країни за правилами ISO-3166 alpha-2. Якщо задано, то AllyMailer обробляє
    /// персональні дані проєкту відповідно до законів заданої країни — наприклад, відповідно до GDPR для
    /// європейських країн.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Дата й час створення проєкту в часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(rename = "reg_time", default, skip_serializing_if = "Option::is_none")]
    pub reg_time: Option<String>,
    /// Чи дозволено надсилання листів для проєкту.
    /// За замовчуванням: `True`.
    #[serde(
        rename = "send_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_enabled: Option<bool>,
    /// Якщо false, то AllyMailer додає в кінець кожного листа системне посилання відписки. Значення true
    /// вимикає автоматичне додавання системою посилання відписки в кінець листа й дозволяє використовувати
    /// власне посилання відписки або надсилати листи зовсім без посилання відписки. Докладніше див.
    /// [тут](https://docs.allymailer.com.ua/ua/unsubscribe-link). Якщо ви не вказуєте значення
    /// custom_unsubscribe_url_enabled під час створення проєкту — то це значення копіюється з головного
    /// облікового запису (якщо в головному обліковому записі було встановлено примусове вимкнення посилання
    /// відписки — то й у проєкту посилання відписки буде вимкнено).
    #[serde(
        rename = "custom_unsubscribe_url_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_unsubscribe_url_enabled: Option<bool>,
    /// Унікальний ідентифікатор backend-домену, який визначає [домен посилань](/tracking-domains) або пул
    /// [виділених IP-адрес](/dedicated-ip) за замовчуванням. Якщо відсутній — обліковому запису/проєкту
    /// буде призначено дефолтний backend-домен за замовчуванням.
    #[serde(
        rename = "backend_domain_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_domain_id: Option<i64>,
    /// Значення лічильника листів для проєкту, збільшується під час надсилання кожного листа.
    #[serde(
        rename = "email_counter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter: Option<i64>,
    /// Дозволяє обмежити кількість надісланих проєктом листів. У разі досягнення цього ліміту надсилання
    /// припиняється. Значення 0 (за замовчуванням) означає відсутність ліміту.
    #[serde(
        rename = "email_counter_limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_limit: Option<i64>,
    /// 'default' - лічильник автоматично скидається в нуль на початку кожного облікового періоду,
    /// 'permanent' - лічильник ніколи не скидається автоматично.
    /// За замовчуванням: `default`.
    #[serde(
        rename = "email_counter_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_counter_mode: Option<crate::common::model::EmailCounterMode>,
    /// Унікальний ідентифікатор, який визначає сторінку відписки за замовчуванням. Якщо відсутній —
    /// обліковому запису/проєкту буде призначено системну сторінку відписки за замовчуванням.
    #[serde(
        rename = "unsubscribe_page_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_page_id: Option<i64>,
}

/// ProjectDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeleteRequest {
    /// Ідентифікатор проєкту, що видаляється. Проєкт має бути ідентифіковано або цим полем project_id, або
    /// полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Необов'язковий API-ключ проєкту, що видаляється. Проєкт має бути ідентифіковано або полем
    /// project_id, або полем project_api_key. Ми рекомендуємо використовувати project_id, це безпечніше.
    #[serde(
        rename = "project_api_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_api_key: Option<String>,
}
