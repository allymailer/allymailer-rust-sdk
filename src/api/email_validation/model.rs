//! Models of the EmailValidation section.

use serde::{Deserialize, Serialize};

/// EmailValidationSingleRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailValidationSingleRequest {
    /// Email-адреса, яку необхідно перевірити.
    #[serde(rename = "email", default)]
    pub email: String,
}

/// EmailValidationSingleResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailValidationSingleResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Email, що перевіряється.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Можливі статуси:
    ///
    /// - "valid" - email-адреса дійсна,
    /// - "invalid" - email-адреса недійсна,
    /// - "suspicious" - email-адреса підозріла,
    /// - "unknown" - адресу не вдалося перевірити. Приймаючий сервер не відповів у межах припустимого часу перевірки.
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Можливі статуси:
    ///
    /// - "no_mx_record" - у домену відсутні MX-записи,
    /// - "syntax_error" - адреса не відповідає стандартам email,
    /// - "possible_typo" - можливо, адресу введено з помилкою,
    /// - "mailbox_not_found" - адреса не існує,
    /// - "global_suppression" - адреса перманентно недоступна в сервісі через багаторазові недоставки,
    /// - "disposable" - одноразовий email, зазвичай такі адреси приймають пошту лише декілька хвилин,
    /// - "role" - загальна адреса електронної пошти, найчастіше використовується групою людей і не належить конкретній особі,
    /// - "abuse" - адреси, з яких надходить велика кількість скарг і натискань на "спам", іноді навіть в автоматичному режимі,
    /// - "spamtrap" - адреси-спам-пастки, зазвичай розміщуються поштовими провайдерами у відкритому доступі, але ніколи не використовуються для підписки. Надсилання листів на такі адреси сильно шкодить репутації відправника,
    /// - "smtp_connection_failed" - SMTP-сервер недоступний — можливо, адресу введено з помилкою.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Імовірність того, що адреса дійсна. Від 0 до 100 (100 - дійсна, 0 - недійсна).
    #[serde(rename = "validity", default, skip_serializing_if = "Option::is_none")]
    pub validity: Option<i64>,
    /// Локальна частина email-адреси (усе, що перед знаком "@").
    #[serde(
        rename = "local_part",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_part: Option<String>,
    /// Доменна частина email-адреси.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Чи є у домену MX-запис? true якщо є, false якщо відсутній.
    #[serde(rename = "mx_found", default, skip_serializing_if = "Option::is_none")]
    pub mx_found: Option<bool>,
    /// Бажана MX-запис домену.
    #[serde(rename = "mx_record", default, skip_serializing_if = "Option::is_none")]
    pub mx_record: Option<String>,
    /// Можлива адреса, отримана після виправлення помилки (для cause=possible_typo).
    #[serde(
        rename = "did_you_mean",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub did_you_mean: Option<String>,
    /// Дата й час перевірки адреси у форматі РРРР-ММ-ДД гг:хх:сс у часовому поясі UTC.
    #[serde(
        rename = "processed_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_at: Option<String>,
}
