//! EventDump section: EventDumpApi and its models.

pub mod model;

use crate::client::{ApiClient, ApiError};

/// Методи для отримання списку подій, що сталися, за певний період. Дані зберігаються до 45 днів,
/// залежно від вашого тарифу. Це означає, що можна запросити події, починаючи з 00:00:00 45-го дня тому
/// і до поточного моменту часу. Для індивідуальних тарифів з великими обсягами за замовчуванням події
/// доступні лише за останні 7 днів.
#[derive(Debug, Clone, Copy)]
pub struct EventDumpApi<'a> {
    client: &'a ApiClient,
}

impl<'a> EventDumpApi<'a> {
    /// Creates the EventDump API.
    pub fn new(client: &'a ApiClient) -> Self {
        EventDumpApi { client }
    }

    /// Асинхронний метод, який ініціює підготовку для завантаження файлів дампа у форматі CSV. Максимальна
    /// кількість збережених/створюваних дампів обмежена десятьма. У разі досягнення ліміту для запиту на
    /// підготовку нового дампа потрібно видалити старі. Час зберігання сформованого дампа — 8 годин, після
    /// чого він автоматично видаляється. Якщо період запиту даних більший за добу або подій багато, буде
    /// сформовано декілька файлів, у кожному з яких буде не більше ніж 100 000 подій. Файл містить колонки
    /// з такою інформацією — 'event_time', 'job_id', 'from_email', 'email', 'status', 'delivery_status',
    /// 'metadata', 'destination_response', 'user_agent', 'url', 'ip', 'tags' і 'project_id' (якщо у
    /// користувача увімкнено проєкти). Ви можете відфільтрувати поля та змінити їх порядок, а також додати
    /// поле 'subject' за допомогою параметра 'dump_fileds'.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create>
    pub async fn create(
        &self,
        request: &model::EventDumpCreateRequest,
    ) -> Result<model::EventDumpCreateResponse, ApiError> {
        self.client
            .call("/event-dump/create.json", serde_json::to_value(request)?)
            .await
    }

    /// Повертає властивості дампа за його ідентифікатором.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-get>
    pub async fn get(
        &self,
        request: &model::EventDumpGetRequest,
    ) -> Result<model::EventDumpGetResponse, ApiError> {
        self.client
            .call("/event-dump/get.json", serde_json::to_value(request)?)
            .await
    }

    /// Метод без параметрів, повертає список усіх дампів подій.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-list>
    pub async fn list(&self) -> Result<model::EventDumpListResponse, ApiError> {
        self.client
            .call("/event-dump/list.json", serde_json::Value::Null)
            .await
    }

    /// Видаляє дамп із черги та/або зі сховища.
    ///
    /// Документація: <https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-delete>
    pub async fn delete(
        &self,
        request: &model::EventDumpDeleteRequest,
    ) -> Result<crate::common::model::SuccessResponse, ApiError> {
        self.client
            .call("/event-dump/delete.json", serde_json::to_value(request)?)
            .await
    }
}
