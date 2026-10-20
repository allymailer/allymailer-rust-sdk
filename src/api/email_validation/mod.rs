//! EmailValidation section: EmailValidationApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Методи перевірки email-адрес. Перевірка адреси електронної пошти надає докладну інформацію про
/// працездатність email. Це дозволить виключити з надсилання неіснуючі email, знизити показник скарг,
/// очистити список підписників і врешті-решт покращити вашу репутацію як відправника. Наприклад, ви
/// можете інтегрувати процес перевірки у форму реєстрації вашого сайту і вже на етапі введення адреси
/// повідомляти користувачам, що адреса, яку вони ввели у форму, недійсна.
#[derive(Debug, Clone, Copy)]
pub struct EmailValidationApi<'a> {
    client: &'a ApiClient,
}

impl<'a> EmailValidationApi<'a> {
    /// Creates the EmailValidation API.
    pub fn new(client: &'a ApiClient) -> Self {
        EmailValidationApi { client }
    }

    /// Допускається не більше ніж два одночасні запити; у разі перевищення буде повернуто помилку. Оскільки
    /// перевірка має бути швидкою, ми обмежуємо час, що витрачається на перевірку адреси електронної пошти.
    /// Тому, якщо ми зіткнемося з повільним поштовим сервером, час відповіді якого перевищує 5 секунд,
    /// результат перевірки може бути невідомим.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#email-validation-single>
    pub async fn single(
        &self,
        request: &model::EmailValidationSingleRequest,
    ) -> Result<model::EmailValidationSingleResponse, ApiError> {
        self.client
            .call(
                "/email-validation/single.json",
                serde_json::to_value(request)?,
            )
            .await
    }
}
