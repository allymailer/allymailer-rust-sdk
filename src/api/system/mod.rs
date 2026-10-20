//! System section: SystemApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Системні методи.
#[derive(Debug, Clone, Copy)]
pub struct SystemApi<'a> {
    client: &'a ApiClient,
}

impl<'a> SystemApi<'a> {
    /// Creates the System API.
    pub fn new(client: &'a ApiClient) -> Self {
        SystemApi { client }
    }

    /// Перевіряє валідність API-ключа та повертає ідентифікатор користувача.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#system-ping>
    pub async fn ping(&self) -> Result<model::SystemPingResponse, ApiError> {
        self.client
            .call("/system/ping.json", serde_json::Value::Null)
            .await
    }

    /// Повертає інформацію про користувача або проєкт за API-ключем.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#system-info>
    pub async fn info(&self) -> Result<model::SystemInfoResponse, ApiError> {
        self.client
            .call("/system/info.json", serde_json::Value::Null)
            .await
    }
}
