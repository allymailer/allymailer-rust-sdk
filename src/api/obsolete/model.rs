//! Models of the Obsolete section.

use serde::{Deserialize, Serialize};

/// Подробиці відписки:
///
/// - "unsubscribed" якщо email було щойно відписано,
/// - "was inactive" якщо він уже був позначений як неактивний і немає сенсу відписувати,
/// - "was unsubscribed" якщо email уже було відписано раніше,
/// - "added and unsubscribed" якщо для цього email не зберігалося жодного більш раннього статусу, тому його було додано й одразу ж відписано.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ObsoleteUnsubscribedSetMessage {
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "was inactive")]
    WasInactive,
    #[serde(rename = "was unsubscribed")]
    WasUnsubscribed,
    #[serde(rename = "added and unsubscribed")]
    AddedAndUnsubscribed,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// ObsoleteUnsubscribedSetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedSetRequest {
    /// Email, який має бути відписано.
    #[serde(rename = "address", default)]
    pub address: String,
}

/// ObsoleteUnsubscribedSetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedSetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Відписаний email.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Подробиці відписки:
    ///
    /// - "unsubscribed" якщо email було щойно відписано,
    /// - "was inactive" якщо він уже був позначений як неактивний і немає сенсу відписувати,
    /// - "was unsubscribed" якщо email уже було відписано раніше,
    /// - "added and unsubscribed" якщо для цього email не зберігалося жодного більш раннього статусу, тому його було додано й одразу ж відписано.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ObsoleteUnsubscribedSetMessage>,
}

/// ObsoleteUnsubscribedCheckRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedCheckRequest {
    /// Email, який необхідно перевірити.
    #[serde(rename = "address", default)]
    pub address: String,
}

/// ObsoleteUnsubscribedCheckResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedCheckResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Email, який було перевірено.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// true якщо відписано, false якщо ні.
    #[serde(
        rename = "is_unsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_unsubscribed: Option<bool>,
}

/// ObsoleteUnsubscribedListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedListRequest {
    /// Дата у форматі РРРР-ММ-ДД, щоб отримати всі відписані адреси з "date_from" по сьогоднішній день.
    /// Якщо параметр не вказати, період за замовчуванням буде з 00:00 сьогоднішнього дня за UTC.
    #[serde(rename = "date_from", default, skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
}

/// ObsoleteUnsubscribedListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів з даними відписаних.
    #[serde(
        rename = "unsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed: Option<Vec<ObsoleteUnsubscribedListResponseUnsubscribedItem>>,
}

/// ObsoleteUnsubscribedListResponseUnsubscribedItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObsoleteUnsubscribedListResponseUnsubscribedItem {
    /// Відписаний email.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Дата й час відписки адреси у форматі РРРР-ММ-ДД гг:хх:сс у часовому поясі UTC.
    #[serde(
        rename = "unsubscribed_on",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed_on: Option<String>,
}
