//! Models of the Domain section.

use serde::{Deserialize, Serialize};

/// Лише домени зі статусом DKIM-запису "active" можуть використовуватися для надсилання листів.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DkimStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// DomainGetDnsRecordsRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainGetDnsRecordsRequest {
    /// Домен, для якого потрібно отримати інформацію про DNS-записи.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainGetDnsRecordsResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainGetDnsRecordsResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Домен, для якого потрібно отримати інформацію про DNS-записи.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Запис, який потрібно внести "як є" для підтвердження володіння цим доменом.
    #[serde(
        rename = "verification-record",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_record: Option<String>,
    /// DKIM-підпис для домену. У цьому полі повертається лише ключ. Щоб TXT-запис був коректним, перед
    /// ключем потрібно додати "v=DKIM1; k=rsa; p=".
    #[serde(rename = "dkim", default, skip_serializing_if = "Option::is_none")]
    pub dkim: Option<String>,
    /// Значення CNAME-запису для хоста "_dmarc" домену.
    #[serde(rename = "dmarc", default, skip_serializing_if = "Option::is_none")]
    pub dmarc: Option<String>,
}

/// DomainValidateVerificationRecordRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateVerificationRecordRequest {
    /// Домен, для якого перевіряється запис про володіння.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainValidateVerificationRecordResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateVerificationRecordResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Налагоджувальне повідомлення про хід перевірки.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// DomainValidateDkimRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateDkimRequest {
    /// Домен, для якого перевіряється DKIM-запис.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainValidateDkimResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainValidateDkimResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Налагоджувальне повідомлення про хід перевірки.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// DomainListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListRequest {
    /// Опціональне ім'я домену, якщо вам потрібно отримати статуси лише за одним доменом.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Максимальна кількість доменів, що повертаються. За замовчуванням 50.
    /// За замовчуванням: `50`.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Індекс першого домену зі списку, що повертається, рахуючи з нуля.
    /// За замовчуванням: `0`.
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// DomainListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів, що описують домени.
    #[serde(rename = "domains", default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainListResponseDomainsItem>>,
}

/// DomainListResponseDomainsItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponseDomainsItem {
    /// Ім'я домену.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Об'єкт, що описує запис перевірки володіння та її статус.
    #[serde(
        rename = "verification-record",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_record: Option<DomainListResponseDomainsItemVerificationRecord>,
    /// Об'єкт, що описує DKIM-запис та її статус.
    #[serde(rename = "dkim", default, skip_serializing_if = "Option::is_none")]
    pub dkim: Option<DomainListResponseDomainsItemDkim>,
}

/// Об'єкт, що описує запис перевірки володіння та її статус.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponseDomainsItemVerificationRecord {
    /// Запис, який потрібно внести "як є" для підтвердження володіння цим доменом.
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Лише домени зі статусом "confirmed" запису про володіння доменом можуть використовуватися для
    /// надсилання листів.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::VerificationRecordStatus>,
}

/// Об'єкт, що описує DKIM-запис та її статус.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainListResponseDomainsItemDkim {
    /// DKIM-підпис для домену. У цьому полі повертається лише ключ. Щоб DNS-запис був коректним, перед
    /// ключем потрібно додати "k=rsa, p=" (див.
    /// [domain/get-dns-records](https://docs.allymailer.com.ua/ua/web-api-ref#domain-get-dns-records)).
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Лише домени зі статусом DKIM-запису "active" можуть використовуватися для надсилання листів.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DkimStatus>,
}

/// DomainGetSandboxDomainResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainGetSandboxDomainResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Ім'я sandbox-домену.
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// DomainDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDeleteRequest {
    /// Ім'я домену, який потрібно видалити.
    #[serde(rename = "domain", default)]
    pub domain: String,
}

/// DomainDeleteResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDeleteResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Домен видалено
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
