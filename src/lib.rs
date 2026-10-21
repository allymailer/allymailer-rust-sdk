//! AllyMailer Web API client.
//!
//! ```ignore
//! use allymailer::api::email::model::EmailSendRequest;
//! use allymailer::api::email::model::MessageObject;
//! use allymailer::api::email::model::MessageObjectRecipientsItem;
//! use allymailer::common::model::BodyObject;
//! use allymailer::common::model::TemplateEngine;
//! use allymailer::AllyMailerClient;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), allymailer::ApiError> {
//!     let client = AllyMailerClient::new("YOUR_API_KEY");
//!
//!     let response = client
//!         .email()
//!         .send(&EmailSendRequest {
//!             message: MessageObject {
//!                 recipients: vec![
//!                     MessageObjectRecipientsItem {
//!                         email: "user@example.com".to_string(),
//!                         substitutions: Some(HashMap::from([
//!                             ("CustomerId".to_string(), "12452".to_string()),
//!                             ("to_name".to_string(), "John Smith".to_string()),
//!                         ])),
//!                         metadata: Some(HashMap::from([
//!                             ("campaign_id".to_string(), "c77f4f4e-3561-49f7-9f07-c35be01b4f43".to_string()),
//!                             ("customer_hash".to_string(), "b253ac7".to_string()),
//!                         ])),
//!                     },
//!                 ],
//!                 template_id: Some("00000000-0000-0000-0000-000000000000".to_string()),
//!                 global_language: Some("en".to_string()),
//!                 template_engine: Some(TemplateEngine::Velocity),
//!                 body: BodyObject {
//!                     html: Some("<b>Hello, {{to_name}}</b>".to_string()),
//!                     plaintext: Some("Hello, {{to_name}}".to_string()),
//!                     amp: Some("<!doctype html><html amp4email><head> <meta charset=\"utf-8\"><script async src=\"https://cdn.ampproject.org/v0.js\"></script> <style amp4email-boilerplate>body{visibility:hidden}</style></head><body> Hello, AMP4EMAIL world.</body></html>".to_string()),
//!                 },
//!                 subject: "AllyMailer test email ".to_string(),
//!                 from_email: Some("email@example.com".to_string()),
//!                 from_name: Some("John Smith".to_string()),
//!                 reply_to: Some("email@example.com".to_string()),
//!                 reply_to_name: Some("John Smith".to_string()),
//!                 idempotence_key: Some("SG1VsbG68sIH2dvc5mx890kIQ".to_string()),
//!                 ..Default::default()
//!             },
//!         })
//!         .await?;
//!     println!("{:?}", response.job_id);
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod common;

pub use client::{ApiClient, ApiError, Configuration};
pub use serde_json;

/// Entry point of the AllyMailer API: one accessor per section.
#[derive(Debug, Clone)]
pub struct AllyMailerClient {
    api_client: ApiClient,
}

impl AllyMailerClient {
    /// Client with the default configuration.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(Configuration::new(api_key))
    }

    /// Client for an API host (`client::HOST_CP`, "cp.allymailer.com") or a full base URL.
    pub fn with_host(api_key: impl Into<String>, host: &str) -> Self {
        let mut config = Configuration::new(api_key);
        config.base_url = client::base_url_for(host);
        Self::with_config(config)
    }

    /// Client with a custom configuration.
    pub fn with_config(config: Configuration) -> Self {
        Self::from_api_client(ApiClient::new(config))
    }

    /// Client on a custom transport, e.g. with a custom reqwest client (proxy, TLS):
    /// `ApiClient::with_http_client(config, http)`.
    pub fn from_api_client(api_client: ApiClient) -> Self {
        AllyMailerClient { api_client }
    }

    /// Low level transport.
    pub fn api_client(&self) -> &ApiClient {
        &self.api_client
    }

    /// Ця група методів пов'язана з надсиланням email.
    pub fn email(&self) -> api::email::EmailApi<'_> {
        api::email::EmailApi::new(&self.api_client)
    }

    /// Методи перевірки email-адрес. Перевірка адреси електронної пошти надає докладну інформацію про
    /// працездатність email. Це дозволить виключити з надсилання неіснуючі email, знизити показник скарг,
    /// очистити список підписників і врешті-решт покращити вашу репутацію як відправника. Наприклад, ви
    /// можете інтегрувати процес перевірки у форму реєстрації вашого сайту і вже на етапі введення адреси
    /// повідомляти користувачам, що адреса, яку вони ввели у форму, недійсна.
    pub fn email_validation(&self) -> api::email_validation::EmailValidationApi<'_> {
        api::email_validation::EmailValidationApi::new(&self.api_client)
    }

    /// Керування email-адресами, підтвердженими для надсилання із sandbox-домену. Методи доступні лише з
    /// API-ключем основного облікового запису.
    pub fn confirmed_email(&self) -> api::confirmed_email::ConfirmedEmailApi<'_> {
        api::confirmed_email::ConfirmedEmailApi::new(&self.api_client)
    }

    /// Ви можете зберегти шаблон на боці AllyMailer і потім посилатися на нього за ідентифікатором. Це не
    /// обов'язково, але вкрай рекомендовано у разі повторюваних email-повідомлень або за великих обсягів
    /// надсилання. AllyMailer підтримує два різні синтаксиси для шаблонів, докладніше читайте
    /// [тут](https://docs.allymailer.com.ua/ua/template-engines).
    pub fn template(&self) -> api::template::TemplateApi<'_> {
        api::template::TemplateApi::new(&self.api_client)
    }

    /// AllyMailer дозволяє вам отримувати сповіщення про зміну статусу доставки листа або спам-блокування.
    /// Ви надаєте URL, і в разі виникнення події, що вас цікавить, ми сповіщаємо вас, передаючи на цей URL
    /// JSON-дані про подію.
    pub fn webhook(&self) -> api::webhook::WebhookApi<'_> {
        api::webhook::WebhookApi::new(&self.api_client)
    }

    /// Методи для керування адресами, надсилання на які заблоковано з різних причин — відписка,
    /// недоступність, скарги.
    pub fn suppression(&self) -> api::suppression::SuppressionApi<'_> {
        api::suppression::SuppressionApi::new(&self.api_client)
    }

    /// Методи для налаштування доменів надсилання та перевірки їх статусів. Корисні, якщо вам недостатньо
    /// додавання доменів через вебінтерфейс AllyMailer і, наприклад, хочеться автоматизувати додавання
    /// доменів для ваших власних клієнтів.
    pub fn domain(&self) -> api::domain::DomainApi<'_> {
        api::domain::DomainApi::new(&self.api_client)
    }

    /// Методи для отримання списку подій, що сталися, за певний період. Дані зберігаються до 45 днів,
    /// залежно від вашого тарифу. Це означає, що можна запросити події, починаючи з 00:00:00 45-го дня тому
    /// і до поточного моменту часу. Для індивідуальних тарифів з великими обсягами за замовчуванням події
    /// доступні лише за останні 7 днів.
    pub fn event_dump(&self) -> api::event_dump::EventDumpApi<'_> {
        api::event_dump::EventDumpApi::new(&self.api_client)
    }

    /// Методи керування користувацькими мітками. За допомогою міток ви можете класифікувати листи за
    /// потрібними критеріями. Відокремити маркетингові листи від транзакційних або листи з підтвердженням
    /// реєстрації від сповіщень про замовлення. AllyMailer дозволяє позначати мітками кожне вихідне
    /// повідомлення, а потім сегментувати й аналізувати статистику на основі зазначених під час надсилання
    /// міток.
    pub fn tag(&self) -> api::tag::TagApi<'_> {
        api::tag::TagApi::new(&self.api_client)
    }

    /// Проєкти дозволяють об'єднати кілька незалежних доменів із webhook, шаблонами та списками відписки.
    /// Це дуже корисно, якщо у вас є власні клієнти і ви керуєте їхніми даними з одного облікового запису
    /// AllyMailer. Кожен проєкт отримує власний ключ API, а виклики методів API з цим ключем обробляють
    /// лише дані цього проєкту. Таким чином, ви можете призначити окремий проєкт для кожного користувача і
    /// бути впевненими, що вони не конфліктуватимуть: наприклад, відписка від одного проєкту не вплине на
    /// інший.
    pub fn project(&self) -> api::project::ProjectApi<'_> {
        api::project::ProjectApi::new(&self.api_client)
    }

    /// Системні методи.
    pub fn system(&self) -> api::system::SystemApi<'_> {
        api::system::SystemApi::new(&self.api_client)
    }

    /// Застарілі методи роботи з відписками. Ми рекомендуємо користуватися замість них гнучкішими методами
    /// [suppression](https://docs.allymailer.com.ua/ua/web-api-ref#suppression). Зверніть увагу, що знову
    /// підписати відписаного адресата через WebAPI не можна. Можна лише надіслати йому email із посиланням
    /// повторної підписки за допомогою методу email/subscribe.
    pub fn obsolete(&self) -> api::obsolete::ObsoleteApi<'_> {
        api::obsolete::ObsoleteApi::new(&self.api_client)
    }
}
