//! ConfirmedEmail section: ConfirmedEmailApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Керування email-адресами, підтвердженими для надсилання із sandbox-домену. Методи доступні лише з
/// API-ключем основного облікового запису.
#[derive(Debug, Clone, Copy)]
pub struct ConfirmedEmailApi<'a> {
    client: &'a ApiClient,
}

impl<'a> ConfirmedEmailApi<'a> {
    /// Creates the ConfirmedEmail API.
    pub fn new(client: &'a ApiClient) -> Self {
        ConfirmedEmailApi { client }
    }

    /// Adds an email address and sends a confirmation link.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#confirmed-email-create>
    pub async fn create(
        &self,
        request: &model::ConfirmedEmailRequest,
    ) -> Result<model::ConfirmedEmailCreateResponse, ApiError> {
        self.client
            .call(
                "/confirmed-email/create.json",
                serde_json::to_value(request)?,
            )
            .await
    }

    /// Lists confirmed and unconfirmed email addresses for the account.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#confirmed-email-list>
    pub async fn list(&self) -> Result<model::ConfirmedEmailListResponse, ApiError> {
        self.client
            .call("/confirmed-email/list.json", serde_json::Value::Null)
            .await
    }

    /// Deletes an email address from the account's confirmed email list.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#confirmed-email-delete>
    pub async fn delete(
        &self,
        request: &model::ConfirmedEmailRequest,
    ) -> Result<model::ConfirmedEmailDeleteResponse, ApiError> {
        self.client
            .call(
                "/confirmed-email/delete.json",
                serde_json::to_value(request)?,
            )
            .await
    }
}
