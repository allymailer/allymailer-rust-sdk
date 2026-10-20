//! Obsolete section: ObsoleteApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Застарілі методи роботи з відписками. Ми рекомендуємо користуватися замість них гнучкішими методами
/// [suppression](https://docs.allymailer.com.ua/ua/web-api-ref#suppression). Зверніть увагу, що знову
/// підписати відписаного адресата через WebAPI не можна. Можна лише надіслати йому email із посиланням
/// повторної підписки за допомогою методу email/subscribe.
#[derive(Debug, Clone, Copy)]
pub struct ObsoleteApi<'a> {
    client: &'a ApiClient,
}

impl<'a> ObsoleteApi<'a> {
    /// Creates the Obsolete API.
    pub fn new(client: &'a ApiClient) -> Self {
        ObsoleteApi { client }
    }

    /// Внести email до списку відписаних.
    ///
    /// Після внесення email до списку відписаних усі майбутні спроби надіслати на нього повідомлення будуть
    /// відхилятися. Зауважте, що видалити email зі списку відписаних безпосередньо за допомогою API не
    /// можна. Але можна надіслати йому повідомлення з посиланням повторної підписки за допомогою методу
    /// [email/subscribe](https://docs.allymailer.com.ua/ua/web-api-ref#email-subscribe) або видалити його
    /// зі списку відписаних вручну на сторінці [Пошук email](https://cp.allymailer.com/ua/user/email) у
    /// панелі керування (але кількість таких видалень на день сильно обмежена).
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#unsubscribed-set>
    pub async fn unsubscribed_set(
        &self,
        request: &model::ObsoleteUnsubscribedSetRequest,
    ) -> Result<model::ObsoleteUnsubscribedSetResponse, ApiError> {
        self.client
            .call("/unsubscribed/set.json", serde_json::to_value(request)?)
            .await
    }

    /// Перевіряє, чи відписано email.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#unsubscribed-check>
    pub async fn unsubscribed_check(
        &self,
        request: &model::ObsoleteUnsubscribedCheckRequest,
    ) -> Result<model::ObsoleteUnsubscribedCheckResponse, ApiError> {
        self.client
            .call("/unsubscribed/check.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає список адрес, відписаних починаючи з указаної дати.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#unsubscribed-list>
    pub async fn unsubscribed_list(
        &self,
        request: &model::ObsoleteUnsubscribedListRequest,
    ) -> Result<model::ObsoleteUnsubscribedListResponse, ApiError> {
        self.client
            .call("/unsubscribed/list.json", serde_json::to_value(request)?)
            .await
    }
}
