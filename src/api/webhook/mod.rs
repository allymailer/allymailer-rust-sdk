//! Webhook section: WebhookApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// AllyMailer дозволяє вам отримувати сповіщення про зміну статусу доставки листа або спам-блокування.
/// Ви надаєте URL, і в разі виникнення події, що вас цікавить, ми сповіщаємо вас, передаючи на цей URL
/// JSON-дані про подію.
///
/// Ви можете налаштувати формат виклику webhook (просто JSON або JSON, запакований у gzip), які саме
/// події вас цікавлять, чи дозволено групувати кілька подій в одному виклику, чи потрібні вам такі
/// подробиці, як SMTP-відповідь сервера або user agent отримувача тощо.
///
/// Зверніть увагу, що ваш обробник має вміти приймати щонайменше 5 паралельних викликів, до 100 подій у
/// кожному (але ви можете налаштувати максимально дозволену кількість паралельних викликів, збільшивши
/// її до 100).
///
/// Якщо URL, на який надсилається webhook, недоступний (немає відповіді HTTP 200 OK протягом 3 секунд),
/// спроби надсилання webhook на цей URL до отримання очікуваного 200 OK триватимуть протягом 24 годин з
/// інтервалом у 10 хвилин з додатковим параметром retry_count, значення якого збільшуватиметься на 1 з
/// кожним повторним надсиланням webhook. Якщо протягом 24 годин сталося щонайменше 10 подій і жоден із
/// багаторазових викликів webhook про ці події не був успішним, то ми вважаємо обробник непрацездатним
/// і автоматично встановлюємо його статус у неактивний та повідомляємо про це на email, [вказаний в
/// особистому кабінеті](https://cp.allymailer.com/ua/user/alerting/technical/).
///
/// Формат webhook і структура кожної події описані
/// [тут](https://docs.allymailer.com.ua/ua/web-api-ref#callback-format).
#[derive(Debug, Clone, Copy)]
pub struct WebhookApi<'a> {
    client: &'a ApiClient,
}

impl<'a> WebhookApi<'a> {
    /// Creates the Webhook API.
    pub fn new(client: &'a ApiClient) -> Self {
        WebhookApi { client }
    }

    /// Встановлює або змінює властивості [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) —
    /// вашого обробника сповіщень про події. У момент виклику
    /// [webhook/set](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set) ми очікуємо, що до
    /// вказаного вами URL можна звернутися GET-запитом без параметрів і у відповідь надійде 200 OK.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set>
    pub async fn set(
        &self,
        request: &model::WebhookSetRequest,
    ) -> Result<model::WebhookSetResponse, ApiError> {
        self.client
            .call("/webhook/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає властивості [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook)
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#webhook-get>
    pub async fn get(
        &self,
        request: &model::WebhookGetRequest,
    ) -> Result<model::WebhookGetResponse, ApiError> {
        self.client
            .call("/webhook/get.json", serde_json::to_value(request)?)
            .await
    }

    /// Повернути список усіх або кількох [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook)
    /// (обробників сповіщень про події) користувача або проєкту.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#webhook-list>
    pub async fn list(
        &self,
        request: &model::WebhookListRequest,
    ) -> Result<model::WebhookListResponse, ApiError> {
        self.client
            .call("/webhook/list.json", serde_json::to_value(request)?)
            .await
    }

    /// Видалити webhook.
    ///
    /// Ви також можете тимчасово вимкнути webhook без його видалення, встановивши його поле status у
    /// "disabled" за допомогою методу
    /// [webhook/set](https://docs.allymailer.com.ua/ua/web-api-ref#webhook-set).
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#webhook-delete>
    pub async fn delete(
        &self,
        request: &model::WebhookDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/webhook/delete.json", serde_json::to_value(request)?)
            .await
    }

    /// Опис формату webhook.
    pub fn parse_callback(body: &str) -> Result<model::WebhookCallback, ApiError> {
        Ok(serde_json::from_str(body)?)
    }
}
