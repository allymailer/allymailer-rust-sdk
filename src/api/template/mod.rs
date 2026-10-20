//! Template section: TemplateApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Ви можете зберегти шаблон на боці AllyMailer і потім посилатися на нього за ідентифікатором. Це не
/// обов'язково, але вкрай рекомендовано у разі повторюваних email-повідомлень або за великих обсягів
/// надсилання. AllyMailer підтримує два різні синтаксиси для шаблонів, докладніше читайте
/// [тут](https://docs.allymailer.com.ua/en/template-engines).
#[derive(Debug, Clone, Copy)]
pub struct TemplateApi<'a> {
    client: &'a ApiClient,
}

impl<'a> TemplateApi<'a> {
    /// Creates the Template API.
    pub fn new(client: &'a ApiClient) -> Self {
        TemplateApi { client }
    }

    /// Створити або оновити шаблон.
    ///
    /// Обмеження:
    /// * Максимальний розмір запиту — 10 MB.
    /// * Кількість шаблонів обмежено 10000.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#template-set>
    pub async fn set(
        &self,
        request: &model::TemplateSetRequest,
    ) -> Result<model::TemplateSetResponse, ApiError> {
        self.client
            .call("/template/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає властивості шаблону за його ідентифікатором.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#template-get>
    pub async fn get(
        &self,
        request: &model::TemplateGetRequest,
    ) -> Result<model::TemplateGetResponse, ApiError> {
        self.client
            .call("/template/get.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає повний список або частину списку всіх шаблонів.
    ///
    /// Ви можете переглядати великий список частинами, використовуючи параметри "offset" і "limit". Якщо
    /// повернуто менше ніж "limit" шаблонів — значить, ви досягли кінця списку.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#template-list>
    pub async fn list(
        &self,
        request: &model::TemplateListRequest,
    ) -> Result<model::TemplateListResponse, ApiError> {
        self.client
            .call("/template/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Видаляє шаблон за ідентифікатором.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#template-delete>
    pub async fn delete(
        &self,
        request: &model::TemplateDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/template/delete.json", serde_json::to_value(request)?)
            .await
    }
}
