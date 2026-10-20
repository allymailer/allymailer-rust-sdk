//! Suppression section: SuppressionApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Методи для керування адресами, надсилання на які заблоковано з різних причин — відписка,
/// недоступність, скарги.
#[derive(Debug, Clone, Copy)]
pub struct SuppressionApi<'a> {
    client: &'a ApiClient,
}

impl<'a> SuppressionApi<'a> {
    /// Creates the Suppression API.
    pub fn new(client: &'a ApiClient) -> Self {
        SuppressionApi { client }
    }

    /// Додає email-адресу до списку недоступних. Ви завжди зможете пізніше видалити цю адресу зі списку
    /// недоступних методом
    /// [suppression/delete](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-delete).
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#suppression-set>
    pub async fn set(
        &self,
        request: &model::SuppressionSetRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/suppression/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає причину й дату блокування email-адреси.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#suppression-get>
    pub async fn get(
        &self,
        request: &model::SuppressionGetRequest,
    ) -> Result<model::SuppressionGetResponse, ApiError> {
        self.client
            .call("/suppression/get.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає список недоступних, починаючи з указаної дати.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#suppression-list>
    pub async fn list(
        &self,
        request: &model::SuppressionListRequest,
    ) -> Result<model::SuppressionListResponse, ApiError> {
        self.client
            .call("/suppression/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Видаляє email-адресу зі списку недоступних (лише записи з is_deletable=true). Якщо жодного запису
    /// для email не знайдено, повертається помилка API 3003. Якщо записи про недоступність знайдено, але
    /// жодну з них видалити не можна, повертається помилка API 3004. Якщо перевищено ліміт видалень на
    /// добу, повертається помилка API 906.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#suppression-delete>
    pub async fn delete(
        &self,
        request: &model::SuppressionDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/suppression/delete.json", serde_json::to_value(request)?)
            .await
    }
}
