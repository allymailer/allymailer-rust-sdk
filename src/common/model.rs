//! Models of the Common section.

use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Параметр для вибору [шаблонізатора](https://docs.allymailer.com.ua/en/template-engines) — або
/// "simple", або "velocity", або "liquid", або "none". За замовчуванням — "simple".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TemplateEngine {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "velocity")]
    Velocity,
    #[serde(rename = "liquid")]
    Liquid,
    #[serde(rename = "none")]
    None,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// EmailStatus
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EmailStatus {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "clicked")]
    Clicked,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "soft_bounced")]
    SoftBounced,
    #[serde(rename = "hard_bounced")]
    HardBounced,
    #[serde(rename = "spam")]
    Spam,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Лише домени зі статусом "confirmed" запису про володіння доменом можуть використовуватися для
/// надсилання листів.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerificationRecordStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Встановлює режим роботи лічильника листів. Дозволяє вибрати, як має поводитися лічильник:
/// автоматично скидатися чи залишатися постійним. Припустимі значення: default - лічильник автоматично
/// скидається в нуль на початку кожного облікового періоду, permanent - лічильник ніколи не скидається
/// автоматично. Скидання може бути виконане лише вручну користувачем, передавши значення 0 у полі
/// 'email_counter'. Якщо поле не передано в запиті або передано значення null, значення режиму не
/// змінюється.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EmailCounterMode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "permanent")]
    Permanent,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Об'єкт, що містить у собі html-, plaintext- і amp-частини листа. Або html-, або plaintext-частина
/// має бути присутня обов'язково.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BodyObject {
    /// HTML-частина листа
    #[serde(rename = "html", default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Plaintext-частина листа.
    #[serde(rename = "plaintext", default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
    /// Необов'язкова
    /// [AMP](http://amp.dev/documentation/guides-and-tutorials/learn/email_fundamentals/)-частина листа.
    #[serde(rename = "amp", default, skip_serializing_if = "Option::is_none")]
    pub amp: Option<String>,
}

/// Об'єкт, що містить додаткові заголовки листа, максимум 50 заголовків. Префікс “X-” в імені заголовка
/// обов'язковий, заголовки без нього будуть проігноровані, наприклад X-ALLYMAILER-Global-Language,
/// X-ALLYMAILER-Template-Engine. Стандартні заголовки "To", "CC", "BCC" передаються без префікса "X-",
/// але обробляються спеціальним чином, тому мають низку обмежень, докладніше про це можна прочитати
/// [тут](/cc-and-bcc). Також якщо [техпідтримка](https://cp.allymailer.com/ua/support) схвалила вам
/// вимкнення посилання відписки, то ви також можете передавати заголовки List-Unsubscribe,
/// List-Subscribe, List-Help, List-Owner, List-Archive, In-Reply-To і References.
///
/// Any other header goes to `extra`: `X-MyHeader`, `CC`, `List-Unsubscribe`, ...; a key of a named
/// field is sent from the field, not from `extra`. Unknown keys of a response are read into `extra`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct HeadersObject {
    /// Заголовок для вибору мови посилання та сторінки відписки. Припустимі значення "be", "de", "en",
    /// "es", "fr", "it", "pl", "pt", "ru", "ua", "kz".
    #[serde(
        rename = "X-ALLYMAILER-Global-Language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_allymailer_global_language: Option<String>,
    /// Заголовок для вибору шаблонізатора — або “simple”, або “velocity”, або “liquid”, або “none”. За
    /// замовчуванням — “simple”. Має пріоритет над значенням, що передається в параметрі “template_engine”.
    #[serde(
        rename = "X-ALLYMAILER-Template-Engine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_allymailer_template_engine: Option<String>,
    /// Keys beyond the named fields.
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

impl Serialize for HeadersObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if let Some(value) = &self.x_allymailer_global_language {
            map.serialize_entry("X-ALLYMAILER-Global-Language", value)?;
        }
        if let Some(value) = &self.x_allymailer_template_engine {
            map.serialize_entry("X-ALLYMAILER-Template-Engine", value)?;
        }
        for (key, value) in &self.extra {
            if !matches!(
                key.as_str(),
                "X-ALLYMAILER-Global-Language" | "X-ALLYMAILER-Template-Engine"
            ) {
                map.serialize_entry(key, value)?;
            }
        }
        map.end()
    }
}

/// AttachmentsArrayItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttachmentsArrayItem {
    /// Тип вкладення, див. [MIME](https://en.wikipedia.org/wiki/MIME). Якщо не впевнені, використовуйте
    /// "application/octet-stream".
    #[serde(rename = "type", default)]
    pub type_: String,
    /// Назва вкладення у форматі: "ім'я.розширення". У разі передачі кількох вкладень їхні назви мають бути
    /// унікальними. У назві вкладень заборонено символ ‘/’.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Вміст файлу у [base64](https://en.wikipedia.org/wiki/Base64). Максимальний розмір файлу 7 Мб
    /// (9786710 байт у base64).
    #[serde(rename = "content", default)]
    pub content: String,
}

/// InlineAttachmentsArrayItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineAttachmentsArrayItem {
    /// Тип вкладення, див. [MIME](https://en.wikipedia.org/wiki/MIME). Якщо не впевнені, використовуйте
    /// "application/octet-stream".
    #[serde(rename = "type", default)]
    pub type_: String,
    /// Ідентифікатор вкладення. Якщо, наприклад, "name"="IMAGECID", то для виведення картинки з цього
    /// вкладення в HTML-тексті листа потрібно вказати <img src="cid:IMAGECID">.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Вміст файлу у [base64](https://en.wikipedia.org/wiki/Base64). Максимальний розмір файлу 7 Мб
    /// (9786710 байт у base64).
    #[serde(rename = "content", default)]
    pub content: String,
}

/// SuccessResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
