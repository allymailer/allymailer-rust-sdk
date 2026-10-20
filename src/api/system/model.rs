//! Models of the System section.

use serde::{Deserialize, Serialize};

/// SystemPingResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemPingResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Унікальний ідентифікатор користувача.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// SystemInfoResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Унікальний ідентифікатор користувача.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Email користувача.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки. Присутній лише якщо
    /// API-ключ, використаний для запиту, був API-ключем проєкту.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Назва проєкту, унікальна в межах облікового запису користувача. Присутня лише якщо API-ключ,
    /// використаний для запиту, був API-ключем проєкту.
    #[serde(
        rename = "project_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    /// Об'єкт із властивостями лічильника листів. Присутній лише якщо API-ключ, використаний для запиту,
    /// був API-ключем проєкту.
    #[serde(
        rename = "project_accounting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_accounting: Option<ProjectAccountingObject>,
    /// Об'єкт з властивостями періоду обліку. Присутній лише якщо API-ключ, використаний для запиту, був
    /// API-ключем користувача.
    #[serde(
        rename = "accounting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accounting: Option<SystemInfoResponseAccounting>,
}

/// Об'єкт із властивостями лічильника листів. Присутній лише якщо API-ключ, використаний для запиту,
/// був API-ключем проєкту.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectAccountingObject {
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
}

/// Об'єкт з властивостями періоду обліку. Присутній лише якщо API-ключ, використаний для запиту, був
/// API-ключем користувача.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoResponseAccounting {
    /// Дата й час початку періоду обліку в часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(
        rename = "period_start",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub period_start: Option<String>,
    /// Дата й час кінця періоду обліку в часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(
        rename = "period_end",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub period_end: Option<String>,
    /// Кількість листів, включених в обліковий період.
    #[serde(
        rename = "emails_included",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emails_included: Option<i64>,
    /// Кількість листів, надісланих за обліковий період. Може бути більшою за emails_included у разі
    /// перевищення.
    #[serde(
        rename = "emails_sent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emails_sent: Option<i64>,
}
