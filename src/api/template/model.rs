//! Models of the Template section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Тип редактора шаблону у вебінтерфейсі, "html" або "visual". За замовчуванням — "html". Створення
/// шаблонів із типом редактора "visual" можливе лише через вебінтерфейс.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EditorType {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "visual")]
    Visual,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Об'єкт з усіма властивостями шаблону.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateSetRequest {
    /// Об'єкт, що містить властивості шаблону.
    #[serde(rename = "template", default)]
    pub template: TemplateObject,
}

/// Об'єкт, що містить властивості шаблону.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateObject {
    /// Унікальний ідентифікатор шаблону. Якщо вказано, система спробує знайти шаблон із указаним id і
    /// відредагувати його. Якщо не задано, буде створено новий шаблон.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Назва шаблону.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Тип редактора шаблону у вебінтерфейсі, "html" або "visual". За замовчуванням — "html". Створення
    /// шаблонів із типом редактора "visual" можливе лише через вебінтерфейс.
    #[serde(
        rename = "editor_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub editor_type: Option<EditorType>,
    /// Параметр для вибору [шаблонізатора](https://docs.allymailer.com.ua/en/template-engines) — або
    /// "simple", або "velocity", або "liquid", або "none". За замовчуванням — "simple".
    #[serde(
        rename = "template_engine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_engine: Option<crate::common::model::TemplateEngine>,
    /// Об'єкт для передачі глобальних підстановок (наприклад, назва компанії). Якщо назви змінних
    /// повторюються в об'єкті користувацьких підстановок "substitutions", значення змінних будуть взяті з
    /// об'єкта "substitutions". Підстановки працюють у параметрах:
    ///
    /// - body.html
    /// - body.plaintext
    /// - body.amp
    /// - subject
    /// - from_name
    /// - options.unsubscribe_url
    ///
    /// Назви змінних можуть містити літери латинського алфавіту, цифри, символ "_". Першим символом може
    /// бути лише літера, використання пробілів не допускається.
    #[serde(
        rename = "global_substitutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_substitutions: Option<HashMap<String, String>>,
    /// Об'єкт для передачі метаданих, спільних для всіх отримувачів, у форматі: "key": "value". Максимальна
    /// кількість ключів: 10; максимальна довжина ключа: 64 символи; максимальна довжина значення ключа:
    /// 1024 символи. Метадані передаються під час виклику
    /// [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook). Для групування листів в одну
    /// розсилку може використовуватися системний ключ "campaign_id" зі значенням у вигляді невід'ємного
    /// десяткового цілого до 128 біт або UUID вигляду "c7703772-453e-4c77-a9ae-9a1b94051b5a". Неправильні
    /// значення будуть прирівняні до '0'
    #[serde(
        rename = "global_metadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_metadata: Option<HashMap<String, String>>,
    /// Об'єкт, що містить у собі html-, plaintext- і amp-частини листа. Або html-, або plaintext-частина
    /// має бути присутня обов'язково.
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<crate::common::model::BodyObject>,
    /// Тема листа.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Email-адреса відправника.
    #[serde(rename = "from_email", default)]
    pub from_email: String,
    /// Ім'я відправника.
    #[serde(rename = "from_name", default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Необов'язкова email-адреса для відповідей (на випадок, якщо відрізняється від адреси відправника)
    #[serde(rename = "reply_to", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Необов'язкове ім'я для відповідей (якщо вказано email reply_to і ви хочете, щоб відображався не лише
    /// цей email, але й ім'я)
    #[serde(
        rename = "reply_to_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_name: Option<String>,
    /// 1 = відстеження переходів за посиланнями увімкнено (значення за замовчуванням), 0=вимкнено. Для
    /// використання track_links = 0 вам потрібно [попросити
    /// техпідтримку](https://cp.allymailer.com/ua/support) увімкнути таку можливість.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "track_links",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_links: Option<i64>,
    /// 1 = відстеження прочитань листа увімкнено (значення за замовчуванням), 0 = вимкнено. Для
    /// використання track_read = 0 вам потрібно [попросити
    /// техпідтримку](https://cp.allymailer.com/ua/support) увімкнути таку можливість.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "track_read",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_read: Option<i64>,
    /// Об'єкт, що містить додаткові заголовки листа, максимум 50 заголовків. Префікс “X-” в імені заголовка
    /// обов'язковий, заголовки без нього будуть проігноровані, наприклад X-ALLYMAILER-Global-Language,
    /// X-ALLYMAILER-Template-Engine. Стандартні заголовки "To", "CC", "BCC" передаються без префікса "X-",
    /// але обробляються спеціальним чином, тому мають низку обмежень, докладніше про це можна прочитати
    /// [тут](/cc-and-bcc). Також якщо [техпідтримка](https://cp.allymailer.com/ua/support) схвалила вам
    /// вимкнення посилання відписки, то ви також можете передавати заголовки List-Unsubscribe,
    /// List-Subscribe, List-Help, List-Owner, List-Archive, In-Reply-To і References.
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::common::model::HeadersObject>,
    /// Необов'язковий масив вкладень.
    #[serde(
        rename = "attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments: Option<Vec<crate::common::model::AttachmentsArrayItem>>,
    /// Масив для передачі inline-вкладень, наприклад, для включення зображень у лист замість завантаження
    /// їх за посиланням.
    #[serde(
        rename = "inline_attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_attachments: Option<Vec<crate::common::model::InlineAttachmentsArrayItem>>,
}

/// TemplateSetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateSetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Об'єкт, що містить властивості шаблону.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateObject>,
}

/// TemplateGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateGetRequest {
    /// Ідентифікатор шаблону.
    #[serde(rename = "id", default)]
    pub id: String,
}

/// TemplateGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateGetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Об'єкт, що містить властивості шаблону.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateObject>,
}

/// TemplateListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateListRequest {
    /// Максимальна кількість шаблонів, що повертаються, за замовчуванням 50.
    /// За замовчуванням: `50`.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Індекс першого шаблону зі списку, що повертається, рахуючи з нуля.
    /// За замовчуванням: `0`.
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// TemplateListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів-шаблонів.
    #[serde(rename = "templates", default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<TemplateListObject>>,
}

/// TemplateListObject
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateListObject {
    /// Унікальний ідентифікатор шаблону.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Назва шаблону.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Тип редактора шаблону у вебінтерфейсі, "html" або "visual". За замовчуванням — "html". Створення
    /// шаблонів із типом редактора "visual" можливе лише через вебінтерфейс.
    #[serde(
        rename = "editor_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub editor_type: Option<EditorType>,
    /// Параметр для вибору [шаблонізатора](https://docs.allymailer.com.ua/en/template-engines) — або
    /// "simple", або "velocity", або "liquid", або "none". За замовчуванням — "simple".
    #[serde(
        rename = "template_engine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_engine: Option<crate::common::model::TemplateEngine>,
    /// Об'єкт для передачі глобальних підстановок (наприклад, назва компанії). Якщо назви змінних
    /// повторюються в об'єкті користувацьких підстановок "substitutions", значення змінних будуть взяті з
    /// об'єкта "substitutions". Підстановки працюють у параметрах:
    ///
    /// - body.html
    /// - body.plaintext
    /// - body.amp
    /// - subject
    /// - from_name
    /// - options.unsubscribe_url
    ///
    /// Назви змінних можуть містити літери латинського алфавіту, цифри, символ "_". Першим символом може
    /// бути лише літера, використання пробілів не допускається.
    #[serde(
        rename = "global_substitutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_substitutions: Option<HashMap<String, String>>,
    /// Об'єкт для передачі метаданих, спільних для всіх отримувачів, у форматі: "key": "value". Максимальна
    /// кількість ключів: 10; максимальна довжина ключа: 64 символи; максимальна довжина значення ключа:
    /// 1024 символи. Метадані передаються під час виклику
    /// [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook). Для групування листів в одну
    /// розсилку може використовуватися системний ключ "campaign_id" зі значенням у вигляді невід'ємного
    /// десяткового цілого до 128 біт або UUID вигляду "c7703772-453e-4c77-a9ae-9a1b94051b5a". Неправильні
    /// значення будуть прирівняні до '0'
    #[serde(
        rename = "global_metadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_metadata: Option<HashMap<String, String>>,
    /// Об'єкт, що містить у собі html-, plaintext- і amp-частини листа. Або html-, або plaintext-частина
    /// має бути присутня обов'язково.
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<crate::common::model::BodyObject>,
    /// Тема листа.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Email-адреса відправника.
    #[serde(
        rename = "from_email",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_email: Option<String>,
    /// Ім'я відправника.
    #[serde(rename = "from_name", default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Необов'язкова email-адреса для відповідей (на випадок, якщо відрізняється від адреси відправника)
    #[serde(rename = "reply_to", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Необов'язкове ім'я для відповідей (якщо вказано email reply_to і ви хочете, щоб відображався не лише
    /// цей email, але й ім'я)
    #[serde(
        rename = "reply_to_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_name: Option<String>,
    /// 1 = відстеження переходів за посиланнями увімкнено (значення за замовчуванням), 0=вимкнено. Для
    /// використання track_links = 0 вам потрібно [попросити
    /// техпідтримку](https://cp.allymailer.com/ua/support) увімкнути таку можливість.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "track_links",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_links: Option<i64>,
    /// 1 = відстеження прочитань листа увімкнено (значення за замовчуванням), 0 = вимкнено. Для
    /// використання track_read = 0 вам потрібно [попросити
    /// техпідтримку](https://cp.allymailer.com/ua/support) увімкнути таку можливість.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "track_read",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_read: Option<i64>,
    /// Об'єкт, що містить додаткові заголовки листа, максимум 50 заголовків. Префікс “X-” в імені заголовка
    /// обов'язковий, заголовки без нього будуть проігноровані, наприклад X-ALLYMAILER-Global-Language,
    /// X-ALLYMAILER-Template-Engine. Стандартні заголовки "To", "CC", "BCC" передаються без префікса "X-",
    /// але обробляються спеціальним чином, тому мають низку обмежень, докладніше про це можна прочитати
    /// [тут](/cc-and-bcc). Також якщо [техпідтримка](https://cp.allymailer.com/ua/support) схвалила вам
    /// вимкнення посилання відписки, то ви також можете передавати заголовки List-Unsubscribe,
    /// List-Subscribe, List-Help, List-Owner, List-Archive, In-Reply-To і References.
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::common::model::HeadersObject>,
    /// Необов'язковий масив вкладень.
    #[serde(
        rename = "attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments: Option<Vec<crate::common::model::AttachmentsArrayItem>>,
    /// Масив для передачі inline-вкладень, наприклад, для включення зображень у лист замість завантаження
    /// їх за посиланням.
    #[serde(
        rename = "inline_attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_attachments: Option<Vec<crate::common::model::InlineAttachmentsArrayItem>>,
    /// Час створення шаблону в часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс"
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Унікальний ідентифікатор користувача.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Назва проєкту, унікальна в межах облікового запису користувача.
    #[serde(
        rename = "project_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
}

/// TemplateDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateDeleteRequest {
    /// Ідентифікатор шаблону.
    #[serde(rename = "id", default)]
    pub id: String,
}
