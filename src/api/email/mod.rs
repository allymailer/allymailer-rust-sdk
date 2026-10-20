//! Email section: EmailApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Ця група методів пов'язана з надсиланням email.
#[derive(Debug, Clone, Copy)]
pub struct EmailApi<'a> {
    client: &'a ApiClient,
}

impl<'a> EmailApi<'a> {
    /// Creates the Email API.
    pub fn new(client: &'a ApiClient) -> Self {
        EmailApi { client }
    }

    /// Метод для надсилання листів вашим підписникам.
    ///
    /// Під час надсилання ви можете використовувати підстановки змінних, шаблони, увімкнути відстеження
    /// прочитань або переходів за посиланнями та багато іншого.
    ///
    /// Обмеження:
    /// * максимальний розмір запиту — 10 MB
    /// * максимальна кількість отримувачів (recipients) — 500 email
    /// * у назві вкладень заборонено символ '/'
    ///
    /// Якщо під час надсилання листа у списку отримувачів будуть некоректно введені, неіснуючі, повторювані
    /// або відписані адреси, вони будуть повернуті у відповіді за допомогою параметра failed_emails, лист
    /// же буде надіслано на всі інші прийнятні адреси. Якщо всі адреси потраплять до failed_emails,
    /// надсилання здійснено не буде, буде повернуто помилку з API-кодом 204.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#email-send>
    pub async fn send(
        &self,
        request: &model::EmailSendRequest,
    ) -> Result<model::EmailSendResponse, ApiError> {
        self.client
            .call("/email/send.json", serde_json::to_value(request)?)
            .await
    }

    /// Надіслати посилання на (пере)підписку.
    ///
    /// Метод дозволяє повторно підписати колишнього підписника, який відписався раніше, був заблокований
    /// через скарги або чия адреса недоступна з якоїсь іншої причини.
    ///
    /// Під час виклику надсилається лист у нейтральному дизайні з посиланням підписки. При переході за
    /// посиланням з email підписник перестає бути недоступним для користувача та/або перестає бути
    /// глобально відписаним.
    ///
    /// За замовчуванням дозволено лише одне надсилання листа повторної підписки на добу. Якщо вам потрібно
    /// більше — напишіть у техпідтримку.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#email-subscribe>
    pub async fn subscribe(
        &self,
        request: &model::EmailSubscribeRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/email/subscribe.json", serde_json::to_value(request)?)
            .await
    }
}
