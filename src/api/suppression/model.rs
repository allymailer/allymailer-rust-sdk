//! Models of the Suppression section.

use serde::{Deserialize, Serialize};

/// SuppressionSetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionSetRequest {
    /// Email-адреса, яку потрібно додати до списку недоступних.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Причина, з якої надсилання на цей email блокується. Одна з таких:
    ///
    /// - __unsubscribed__ - зазначену адресу відписано;
    /// - __temporary_unavailable__ - адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:
    ///
    /// - попереднє надсилання було відхилено сервером отримувача як спам;
    /// - поштова скринька отримувача переповнена або не використовується;
    /// - домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;
    /// - сервер відправника було відхилено через блеклистинг;
    ///
    /// - __permanent_unavailable__ - адреса перманентно недоступна через багаторазові недоставки;
    /// - __complained__ - в одному з попередніх листів адресат натиснув “Це спам”.
    #[serde(rename = "cause", default)]
    pub cause: String,
    /// Коли було встановлено недоступність, у часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// SuppressionGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionGetRequest {
    /// Email-адреса, для якої потрібно отримати дані про недоступність.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Якщо у користувача увімкнено проєкти, all_projects=true дозволить шукати за даними всіх проєктів.
    /// За замовчуванням: `False`.
    #[serde(
        rename = "all_projects",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_projects: Option<bool>,
}

/// SuppressionGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionGetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Email-адреса, для якої було запитано дані про недоступність.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Масив об'єктів з інформацією про недоступність.
    #[serde(
        rename = "suppressions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppressions: Option<Vec<SuppressionGetObject>>,
}

/// Об'єкт з даними про блокування email.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionGetObject {
    /// Унікальний ідентифікатор проєкту, ASCII-рядок до 36 символів завдовжки.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Причина, з якої надсилання на цей email блокується. Одна з таких:
    ///
    /// - __unsubscribed__ - зазначену адресу відписано;
    /// - __temporary_unavailable__ - адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:
    ///
    /// - попереднє надсилання було відхилено сервером отримувача як спам;
    /// - поштова скринька отримувача переповнена або не використовується;
    /// - домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;
    /// - сервер відправника було відхилено через блеклистинг;
    ///
    /// - __permanent_unavailable__ - адреса перманентно недоступна через багаторазові недоставки;
    /// - __complained__ - в одному з попередніх листів адресат натиснув “Це спам”.
    /// - __blocked__ - надсилання на цю адресу заборонено адміністрацією AllyMailer.
    ///
    /// У майбутньому можуть з'явитися й інші причини.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Джерело потрапляння до списку недоступних. Одне з таких:
    ///
    /// - __user__ - дії користувача за допомогою методу [suppression/set](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-set);
    /// - __system__ - надсилання на цю адресу заборонено системою, наприклад через багаторазові недоставки;
    /// - __subscriber__ - у зв'язку з діями отримувача. В одному з попередніх листів адресат натиснув “Це спам” або відписався.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Чи можна видалити запис за допомогою виклику suppression/delete.
    #[serde(
        rename = "is_deletable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deletable: Option<bool>,
    /// Коли було встановлено недоступність, у часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// SuppressionListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListRequest {
    /// Причина, з якої надсилання на цей email блокується. Одна з таких:
    ///
    /// - __unsubscribed__ - зазначену адресу відписано;
    /// - __temporary_unavailable__ - адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:
    ///
    /// - попереднє надсилання було відхилено сервером отримувача як спам;
    /// - поштова скринька отримувача переповнена або не використовується;
    /// - домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;
    /// - сервер відправника було відхилено через блеклистинг;
    ///
    /// - __permanent_unavailable__ - адреса перманентно недоступна через багаторазові недоставки;
    /// - __complained__ - в одному з попередніх листів адресат натиснув “Це спам”.
    /// - __blocked__ - надсилання на цю адресу заборонено адміністрацією AllyMailer.
    ///
    /// У майбутньому можуть з'явитися й інші причини.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Джерело потрапляння до списку недоступних. Одне з таких:
    ///
    /// - __user__ - дії користувача за допомогою методу [suppression/set](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-set);
    /// - __system__ - надсилання на цю адресу заборонено системою, наприклад через багаторазові недоставки;
    /// - __subscriber__ - у зв'язку з діями отримувача. В одному з попередніх листів адресат натиснув “Це спам” або відписався.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Дата у форматі РРРР-ММ-ДД гг:хх:сс, щоб отримати список блокування з "start_time" по сьогоднішній
    /// день. Ігнорується, якщо параметр “cursor” не порожній.
    #[serde(
        rename = "start_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<String>,
    /// Параметр указує, з якої позиції починати вибірку. Має бути порожнім або відсутнім для отримання
    /// першої порції даних; для отримання другої та наступних порцій його необхідно задавати, підставляючи
    /// значення поля "cursor" з попередньої порції.
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Максимум записів, що видаються за один раз, за замовчуванням 50.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// SuppressionListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів з інформацією про недоступність.
    #[serde(
        rename = "suppressions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppressions: Option<Vec<SuppressionListObject>>,
    /// Параметр указує, з якої позиції починати вибірку. Має бути порожнім або відсутнім для отримання
    /// першої порції даних; для отримання другої та наступних порцій його необхідно задавати, підставляючи
    /// значення поля "cursor" з попередньої порції.
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Об'єкт з даними про блокування email.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListObject {
    /// Email-адреса, яка знаходиться в списку недоступних.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Причина, з якої надсилання на цей email блокується. Одна з таких:
    ///
    /// - __unsubscribed__ - зазначену адресу відписано;
    /// - __temporary_unavailable__ - адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:
    ///
    /// - попереднє надсилання було відхилено сервером отримувача як спам;
    /// - поштова скринька отримувача переповнена або не використовується;
    /// - домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;
    /// - сервер відправника було відхилено через блеклистинг;
    ///
    /// - __permanent_unavailable__ - адреса перманентно недоступна через багаторазові недоставки;
    /// - __complained__ - в одному з попередніх листів адресат натиснув “Це спам”.
    /// - __blocked__ - надсилання на цю адресу заборонено адміністрацією AllyMailer.
    ///
    /// У майбутньому можуть з'явитися й інші причини.
    #[serde(rename = "cause", default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Джерело потрапляння до списку недоступних. Одне з таких:
    ///
    /// - __user__ - дії користувача за допомогою методу [suppression/set](https://docs.allymailer.com.ua/ua/web-api-ref#suppression-set);
    /// - __system__ - надсилання на цю адресу заборонено системою, наприклад через багаторазові недоставки;
    /// - __subscriber__ - у зв'язку з діями отримувача. В одному з попередніх листів адресат натиснув “Це спам” або відписався.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Чи можна видалити запис за допомогою виклику suppression/delete.
    #[serde(
        rename = "is_deletable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deletable: Option<bool>,
    /// Коли було встановлено недоступність, у часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(rename = "created", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// SuppressionDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuppressionDeleteRequest {
    /// Email-адреса, яку потрібно видалити зі списку недоступних.
    #[serde(rename = "email", default)]
    pub email: String,
}
