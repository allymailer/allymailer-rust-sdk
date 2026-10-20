//! Project section: ProjectApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Проєкти дозволяють об'єднати кілька незалежних доменів із webhook, шаблонами та списками відписки.
/// Це дуже корисно, якщо у вас є власні клієнти і ви керуєте їхніми даними з одного облікового запису
/// AllyMailer. Кожен проєкт отримує власний ключ API, а виклики методів API з цим ключем обробляють
/// лише дані цього проєкту. Таким чином, ви можете призначити окремий проєкт для кожного користувача і
/// бути впевненими, що вони не конфліктуватимуть: наприклад, відписка від одного проєкту не вплине на
/// інший.
///
/// Методи проєктів за замовчуванням вимкнено через заходи захисту від спаму. Щоб їх увімкнути, вам
/// необхідно звернутися до [служби підтримки](https://cp.allymailer.com/ua/support).
///
/// Зверніть увагу: за замовчуванням домени для надсилання підтверджуються та використовуються незалежно
/// для основного облікового запису й кожного з проєктів. Але є можливість увімкнути для проєктів
/// використання доменів, підтверджених в основному обліковому записі. Просто попросіть про це
/// [техпідтримку](https://cp.allymailer.com/ua/support).
#[derive(Debug, Clone, Copy)]
pub struct ProjectApi<'a> {
    client: &'a ApiClient,
}

impl<'a> ProjectApi<'a> {
    /// Creates the Project API.
    pub fn new(client: &'a ApiClient) -> Self {
        ProjectApi { client }
    }

    /// Створити новий проєкт.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#project-create>
    pub async fn create(
        &self,
        request: &model::ProjectCreateRequest,
    ) -> Result<model::ProjectCreateResponse, ApiError> {
        self.client
            .call("/project/create.json", serde_json::to_value(request)?)
            .await
    }

    /// Змінити властивості проєкту.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#project-update>
    pub async fn update(
        &self,
        request: &model::ProjectUpdateRequest,
    ) -> Result<model::ProjectUpdateResponse, ApiError> {
        self.client
            .call("/project/update.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає список проєктів.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#project-list>
    pub async fn list(
        &self,
        request: &model::ProjectListRequest,
    ) -> Result<model::ProjectListResponse, ApiError> {
        self.client
            .call("/project/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Видалити проєкт.
    ///
    /// Деякий час проєкт не видаляється остаточно, і його ще можна відновити — для цього необхідно
    /// звернутися в техпідтримку.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#project-delete>
    pub async fn delete(
        &self,
        request: &model::ProjectDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/project/delete.json", serde_json::to_value(request)?)
            .await
    }
}
