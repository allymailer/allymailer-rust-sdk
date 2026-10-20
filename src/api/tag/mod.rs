//! Tag section: TagApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Методи керування користувацькими мітками. За допомогою міток ви можете класифікувати листи за
/// потрібними критеріями. Відокремити маркетингові листи від транзакційних або листи з підтвердженням
/// реєстрації від сповіщень про замовлення. AllyMailer дозволяє позначати мітками кожне вихідне
/// повідомлення, а потім сегментувати й аналізувати статистику на основі зазначених під час надсилання
/// міток.
#[derive(Debug, Clone, Copy)]
pub struct TagApi<'a> {
    client: &'a ApiClient,
}

impl<'a> TagApi<'a> {
    /// Creates the Tag API.
    pub fn new(client: &'a ApiClient) -> Self {
        TagApi { client }
    }

    /// Метод без параметрів, повертає список усіх користувацьких міток.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#tag-list>
    pub async fn list(&self) -> Result<model::TagListResponse, ApiError> {
        self.client
            .call("/tag/list.json", serde_json::Value::Null)
            .await
    }

    /// Видаляє мітку.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#tag-delete>
    pub async fn delete(
        &self,
        request: &model::TagDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/tag/delete.json", serde_json::to_value(request)?)
            .await
    }
}
