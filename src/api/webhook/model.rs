//! Models of the Webhook section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Статус webhook, за замовчуванням "active". Статус "disabled" означає, що webhook було вимкнено
/// користувачем, "stopped" означає, що webhook було зупинено системою після 24 годин невдалих викликів
/// (мінімум за 10 різними подіями).
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebhookStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "stopped")]
    Stopped,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Формат сповіщень. "json_post"(за замовчуванням) або "json_post_gzip".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebhookEventFormat {
    #[serde(rename = "json_post")]
    JsonPost,
    #[serde(rename = "json_post_gzip")]
    JsonPostGzip,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Тип даних, що містяться в об'єкті event_data. Або "transactional_email_status", або
/// "transactional_spam_block".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EventName {
    #[serde(rename = "transactional_email_status")]
    TransactionalEmailStatus,
    #[serde(rename = "transactional_spam_block")]
    TransactionalSpamBlock,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Тип пристрою отримувача, визначений за user agent. Присутній для статусів "clicked" і "opened";
/// дорівнює "unknown", якщо тип пристрою визначити не вдалося.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeviceType {
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "unknown")]
    Unknown,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Якість трафіку для статусів "opened" і "clicked". "human" — розпізнано звичайний браузер або
/// поштовий клієнт без машинних ознак; "machine" — бот, сканер посилань або проксі безпеки; "protected"
/// — механізм приватності скриньки/ОС, участь людини підтвердити не можна (Apple Mail Privacy
/// Protection / iCloud Private Relay, Gmail Image Proxy; Apple MPP — це protected, не machine);
/// "unknown" — User-Agent відсутній, пошкоджений або не розпізнаний.
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TrafficType {
    #[serde(rename = "human")]
    Human,
    #[serde(rename = "machine")]
    Machine,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "unknown")]
    Unknown,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Тип спам-блока — заблоковано або один відправляючий SMTP, або одразу багато. У разі блокування
/// одиночного SMTP із загального пулу AllyMailer пробуватиме ще декілька. Властивість присутня лише
/// якщо event_name="transactional_spam_block".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BlockType {
    #[serde(rename = "one_smtp")]
    OneSmtp,
    #[serde(rename = "all_smtp")]
    AllSmtp,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// Це подія блокування чи розблокування. Властивість присутня лише якщо
/// event_name="transactional_spam_block".
///
/// Non-exhaustive: the API may add values, a value unknown to this SDK version is read as
/// `UnknownValue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DomainStatus {
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "unblocked")]
    Unblocked,
    /// A value unknown to this SDK version.
    #[serde(other, skip_serializing)]
    UnknownValue,
}

/// WebhookSetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookSetRequest {
    /// Рядок з URL, на який надсилатиметься запит у разі виникнення події (ідентифікатор обробника). В URL
    /// поки що підтримуються лише ASCII-символи. Якщо вам потрібно використати не-ASCII, сконвертуйте URL у
    /// [Punycode](https://en.wikipedia.org/wiki/Punycode).
    #[serde(rename = "url", default)]
    pub url: String,
    /// Статус webhook, за замовчуванням "active". Статус "disabled" означає, що webhook було вимкнено
    /// користувачем, "stopped" означає, що webhook було зупинено системою після 24 годин невдалих викликів
    /// (мінімум за 10 різними подіями).
    /// За замовчуванням: `active`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
    /// Формат сповіщень. "json_post"(за замовчуванням) або "json_post_gzip".
    /// За замовчуванням: `json_post`.
    #[serde(
        rename = "event_format",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_format: Option<WebhookEventFormat>,
    /// Повертати (1) чи ні (0) докладну інформацію про доставку. Якщо delivery_info=1, повертається
    /// додаткова інформація про відповідь сервера та внутрішній статус доставки в разі статусів доставки
    /// "hard_bounced" і "soft_bounced"; user agent, IP-адреса отримувача, геодані, визначена за user agent
    /// інформація про пристрій (тип і марка пристрою, назва ОС, браузера або поштового клієнта) і
    /// traffic_type (human, machine, protected або unknown) в разі статусів "opened" і "clicked"; URL для
    /// статусу "clicked". Для статусів "delivered", "soft_bounced" і "hard_bounced" AllyMailer також може
    /// повернути sender_ip — IP-адресу SMTP-сервера, з якого було надіслано лист, якщо він доступний.
    /// Допустимі значення: `0`, `1`.
    /// За замовчуванням: `0`.
    #[serde(
        rename = "delivery_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_info: Option<i64>,
    /// 0=в одному виклику може надійти декілька подій, 1=лише одна подія на виклик (не рекомендується).
    /// Допустимі значення: `0`, `1`.
    /// За замовчуванням: `0`.
    #[serde(
        rename = "single_event",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub single_event: Option<i64>,
    /// Максимальна кількість дозволених паралельних запитів до вашого сервера. Чим більше ваш сервер може
    /// обробити — тим краще.
    /// За замовчуванням: `10`.
    #[serde(
        rename = "max_parallel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel: Option<i64>,
    /// Об'єкт, що описує події, про які потрібно сповіщати.
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<WebhookEventsObject>,
}

/// Об'єкт, що описує події, про які потрібно сповіщати.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookEventsObject {
    /// Якщо присутній, то надсилатимуться сповіщення про спам-блокування. У масиві має бути єдиний елемент
    /// із рядком "*".
    #[serde(
        rename = "spam_block",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spam_block: Option<Vec<String>>,
    /// Якщо присутній, то надсилатимуться сповіщення про зміну статусу email. У масиві перелічуються ті
    /// статуси, про які надсилатимуться сповіщення.
    #[serde(
        rename = "email_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_status: Option<Vec<crate::common::model::EmailStatus>>,
}

/// WebhookSetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookSetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Об'єкт з усіма властивостями webhook.
    #[serde(rename = "object", default, skip_serializing_if = "Option::is_none")]
    pub object: Option<WebhookObject>,
}

/// Об'єкт з усіма властивостями webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookObject {
    /// Унікальний ідентифікатор webhook.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// URL webhook.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Статус webhook, за замовчуванням "active". Статус "disabled" означає, що webhook було вимкнено
    /// користувачем, "stopped" означає, що webhook було зупинено системою після 24 годин невдалих викликів
    /// (мінімум за 10 різними подіями).
    /// За замовчуванням: `active`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
    /// Формат сповіщень. "json_post"(за замовчуванням) або "json_post_gzip".
    /// За замовчуванням: `json_post`.
    #[serde(
        rename = "event_format",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_format: Option<WebhookEventFormat>,
    /// Повертати (1) чи ні (0) докладну інформацію про доставку. Якщо delivery_info=1, повертається
    /// додаткова інформація про відповідь сервера та внутрішній статус доставки в разі статусів доставки
    /// "hard_bounced" і "soft_bounced"; user agent, IP-адреса отримувача, геодані, визначена за user agent
    /// інформація про пристрій (тип і марка пристрою, назва ОС, браузера або поштового клієнта) і
    /// traffic_type (human, machine, protected або unknown) в разі статусів "opened" і "clicked"; URL для
    /// статусу "clicked". Для статусів "delivered", "soft_bounced" і "hard_bounced" AllyMailer також може
    /// повернути sender_ip — IP-адресу SMTP-сервера, з якого було надіслано лист, якщо він доступний.
    /// Допустимі значення: `0`, `1`.
    /// За замовчуванням: `0`.
    #[serde(
        rename = "delivery_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_info: Option<i64>,
    /// 0=в одному виклику може надійти декілька подій, 1=лише одна подія на виклик (не рекомендується).
    /// Допустимі значення: `0`, `1`.
    /// За замовчуванням: `0`.
    #[serde(
        rename = "single_event",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub single_event: Option<i64>,
    /// Максимальна кількість дозволених паралельних запитів до вашого сервера. Чим більше ваш сервер може
    /// обробити — тим краще.
    /// За замовчуванням: `10`.
    #[serde(
        rename = "max_parallel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel: Option<i64>,
    /// Час останнього оновлення властивостей webhook у часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс".
    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
    /// Об'єкт, що описує події, про які потрібно сповіщати.
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<WebhookEventsObject>,
}

/// Опис формату webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallback {
    /// MD5-хеш рядкового тіла повідомлення, у якому значення auth замінено на api_key користувача/проєкту,
    /// чий обробник викликається. За допомогою цього отримувач сповіщення може як провести автентифікацію,
    /// так і перевірити цілісність сповіщення.
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    /// Масив з одним елементом, що містить дані про події користувача/проєкту.
    #[serde(
        rename = "events_by_user",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub events_by_user: Option<Vec<WebhookCallbackEventsByUserItem>>,
}

/// WebhookCallbackEventsByUserItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItem {
    /// Унікальний ідентифікатор користувача.
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Ідентифікатор проєкту, присутній, якщо webhook було зареєстровано за допомогою API-ключа проєкту.
    #[serde(
        rename = "project_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_id: Option<String>,
    /// Назва проєкту, присутня, якщо webhook було зареєстровано за допомогою API-ключа проєкту.
    #[serde(
        rename = "project_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_name: Option<String>,
    /// Масив подій, про які сповіщає виклик webhook.
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookCallbackEventsByUserItemEventsItem>>,
}

/// WebhookCallbackEventsByUserItemEventsItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItemEventsItem {
    /// Тип даних, що містяться в об'єкті event_data. Або "transactional_email_status", або
    /// "transactional_spam_block".
    #[serde(
        rename = "event_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_name: Option<EventName>,
    /// Об'єкт з різними властивостями події, що залежать від поля "event_name". Нижче перелічено всі
    /// можливі властивості, спочатку для події "transactional_email_status", потім для події
    /// "transactional_spam_block".
    #[serde(
        rename = "event_data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_data: Option<WebhookCallbackEventsByUserItemEventsItemEventData>,
}

/// Об'єкт з різними властивостями події, що залежать від поля "event_name". Нижче перелічено всі
/// можливі властивості, спочатку для події "transactional_email_status", потім для події
/// "transactional_spam_block".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItemEventsItemEventData {
    /// Ідентифікатор завдання, раніше повернутий методом
    /// [email/send](https://docs.allymailer.com.ua/ua/web-api-ref#email-send). Присутній лише якщо
    /// event_name="transactional_email_status".
    #[serde(rename = "job_id", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Метадані, раніше передані методом
    /// [email/send](https://docs.allymailer.com.ua/ua/web-api-ref#email-send) у полі recipients.metadata
    /// або global_metadata. Присутні лише якщо event_name="transactional_email_status".
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Email, на який було здійснено надсилання. Присутній лише якщо
    /// event_name="transactional_email_status".
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Статус доставки листа. Присутній лише якщо event_name="transactional_email_status". Набуває одного з
    /// таких значень:
    ///
    /// __accepted__ — повідомлення прийнято, але ще не надіслано.
    ///
    /// __sent__ — повідомлення надіслано, але ще не доставлено.
    ///
    /// __delivered__ — повідомлення доставлено. Може змінитися на “opened”, “clicked”, “unsubscribed”,
    /// “subscribed” або “spam”.
    ///
    /// __opened__ — повідомлення доставлено й зареєстровано його прочитання. Може змінитися на “clicked”,
    /// “unsubscribed”, “subscribed” або “spam”.
    ///
    /// __clicked__ — повідомлення доставлено, прочитано, було зареєстровано перехід за одним з посилань у
    /// листі. Може змінитися на “unsubscribed”, “subscribed” або “spam”.
    ///
    /// __unsubscribed__ — повідомлення було доставлено отримувачу й прочитано ним, але користувач
    /// відписався за посиланням у листі. Може змінитися на “subscribed”.
    ///
    /// __subscribed__ — повідомлення було доставлено отримувачу й прочитано ним, але користувач відписався,
    /// а потім знову підписався за посиланням у листі. Може змінитися на “unsubscribed”.
    ///
    /// __soft_bounced__ — тимчасова недоставка повідомлення. AllyMailer продовжує спроби доставки протягом
    /// 2 діб. У разі успішної доставки лист перейде у статус “delivered”, у разі неуспішної — у
    /// “hard_bounced”.
    ///
    /// __hard_bounced__ — доставка не вдалася, спроб доставки більше не буде. Статус остаточний. Можливих
    /// причин невдалої доставки безліч, для їх аналізу можна використати текст відповіді SMTP-сервера з
    /// поля delivery_info.destination_response або нашу внутрішню класифікацію помилок з поля
    /// delivery_info.delivery_status.
    ///
    /// __spam__ — повідомлення доставлено, але позначено як “спам” отримувачем. Статус остаточний. Ось
    /// деякі домени, від яких AllyMailer може отримати й обробити скаргу про спам (використовуючи
    /// технологію FBL): mail.ru, msn.com, outlook.com, hotmail.com, live.com, ukr.net, yahoo.com, aol.com.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::EmailStatus>,
    /// Дата й час події в часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс". Присутній лише якщо
    /// event_name="transactional_email_status".
    #[serde(
        rename = "event_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_time: Option<String>,
    /// URL у разі статусів "opened" і "clicked". Присутній лише якщо
    /// event_name="transactional_email_status".
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Об'єкт з додатковою інформацією про доставку. Присутній лише якщо властивість delivery_info під час
    /// встановлення webhook дорівнювала 1 і якщо event_name="transactional_email_status".
    #[serde(
        rename = "delivery_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_info: Option<WebhookCallbackEventsByUserItemEventsItemEventDataDeliveryInfo>,
    /// Дата й час спам-блока в часовому поясі UTC у форматі "РРРР-ММ-ДД гг:хх:сс". Присутній лише якщо
    /// event_name="transactional_spam_block".
    #[serde(
        rename = "block_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_time: Option<String>,
    /// Тип спам-блока — заблоковано або один відправляючий SMTP, або одразу багато. У разі блокування
    /// одиночного SMTP із загального пулу AllyMailer пробуватиме ще декілька. Властивість присутня лише
    /// якщо event_name="transactional_spam_block".
    #[serde(
        rename = "block_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_type: Option<BlockType>,
    /// Домен, що заблокував надсилання. Властивість присутня лише якщо
    /// event_name="transactional_spam_block".
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Кількість заблокованих вихідних SMTP. Властивість присутня лише якщо
    /// event_name="transactional_spam_block".
    #[serde(
        rename = "SMTP_blocks_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_blocks_count: Option<i64>,
    /// Це подія блокування чи розблокування. Властивість присутня лише якщо
    /// event_name="transactional_spam_block".
    #[serde(
        rename = "domain_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_status: Option<DomainStatus>,
}

/// Об'єкт з додатковою інформацією про доставку. Присутній лише якщо властивість delivery_info під час
/// встановлення webhook дорівнювала 1 і якщо event_name="transactional_email_status".
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookCallbackEventsByUserItemEventsItemEventDataDeliveryInfo {
    /// Внутрішній детальний статус доставки AllyMailer. Ось деякі з можливих статусів:
    ///
    /// __err_user_unknown__ -- адреса не існує (і невідомо, чи існувала колись);
    ///
    /// __err_user_inactive__ -- адреса не використовується;
    ///
    /// __err_will_retry__ -- лист було тимчасово відхилено, пізніше буде здійснено повторну спробу
    /// доставки;
    ///
    /// __err_mailbox_discarded__ -- адресу видалено (тобто колись існувала, але тепер її немає);
    ///
    /// __err_mailbox_full__ -- поштова скринька переповнена;
    ///
    /// __err_spam_rejected__ -- лист відхилено як спам;
    ///
    /// __err_blacklisted__ -- лист було відхилено через наявність IP-адреси або домену відправника в
    /// якомусь із чорних списків;
    ///
    /// __err_too_large__ -- на думку сервера отримувача, лист перевищує допустимий розмір;
    ///
    /// __err_unsubscribed__ -- адресат раніше відписався від ваших розсилок;
    ///
    /// __err_unreachable__ -- адресу позначено як постійно недоступну через багаторазові помилки доставки
    /// на цю адресу;
    ///
    /// __err_skip_letter__ -- надсилання скасовано, оскільки email-адреса тимчасово недоступна;
    ///
    /// __err_domain_inactive__ -- домен не приймає пошту або не існує;
    ///
    /// __err_destination_misconfigured__ -- домен не приймає пошту через неправильне налаштування на боці
    /// отримувача, причому відповідь сервера містить інформацію про усувну причину (наприклад,
    /// використовується непрацюючий блеклист тощо);
    ///
    /// __err_delivery_failed__ -- доставка не вдалася з інших причин;
    ///
    /// __err_spam_skipped__ -- надсилання скасовано через блокування розсилки як спаму;
    ///
    /// __err_lost__ -- лист не було надіслано через неузгодженість його частин, або його було втрачено
    /// через збій на нашому боці. Відправник має повторно надіслати лист самостійно, оскільки оригінал не
    /// зберігся;
    ///
    /// Ми залишаємо за собою право змінювати й додавати внутрішні статуси, тому решту
    /// використовуваних наразі статусів перелічимо лише довідково, без пояснень:
    /// ok_accepted, ok_sent, ok_delivered, ok_read, ok_link_visited, ok_unsubscribed, ok_resubscribed,
    /// ok_spam_folder, ok_fbl,
    /// not_sent, skip_dup_unreachable, skip_dup_temp_unreachable, skip_dup_mailbox_full,
    /// err_spam_removed, err_resend, err_unknown, err_retry_letter, err_src_invalid,
    /// err_dest_invalid, err_not_available, err_internal, err_no_dns, err_no_smtp, err_giveup.
    #[serde(
        rename = "delivery_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_status: Option<String>,
    /// Відповідь SMTP-сервера.
    #[serde(
        rename = "destination_response",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_response: Option<String>,
    /// Рядок user agent отримувача. Присутній лише якщо вдалося його визначити, для статусів "clicked" і
    /// "opened".
    #[serde(
        rename = "user_agent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent: Option<String>,
    /// Тип пристрою отримувача, визначений за user agent. Присутній для статусів "clicked" і "opened";
    /// дорівнює "unknown", якщо тип пристрою визначити не вдалося.
    #[serde(
        rename = "device_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_type: Option<DeviceType>,
    /// Марка пристрою отримувача, визначена за user agent. Присутня для статусів "clicked" і "opened", якщо
    /// її вдалося визначити. Для device_type "mobile" і "tablet" присутня завжди та дорівнює "unknown",
    /// якщо марку визначити не вдалося.
    #[serde(
        rename = "device_brand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub device_brand: Option<String>,
    /// Назва операційної системи пристрою отримувача, визначена за user agent. Присутня для статусів
    /// "clicked" і "opened"; дорівнює "unknown", якщо ОС визначити не вдалося.
    #[serde(rename = "os_name", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Назва браузера, у якому було відкрито лист, визначена за user agent. Присутня для статусів "clicked"
    /// і "opened", лише якщо було визначено браузер, тому ніколи не надходить разом з email_client_name.
    #[serde(
        rename = "browser_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_name: Option<String>,
    /// Назва поштового клієнта, у якому було відкрито лист, визначена за user agent. Присутня для статусів
    /// "clicked" і "opened", лише якщо було визначено поштовий клієнт, тому ніколи не надходить разом з
    /// browser_name.
    #[serde(
        rename = "email_client_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_client_name: Option<String>,
    /// Якість трафіку для статусів "opened" і "clicked". "human" — розпізнано звичайний браузер або
    /// поштовий клієнт без машинних ознак; "machine" — бот, сканер посилань або проксі безпеки; "protected"
    /// — механізм приватності скриньки/ОС, участь людини підтвердити не можна (Apple Mail Privacy
    /// Protection / iCloud Private Relay, Gmail Image Proxy; Apple MPP — це protected, не machine);
    /// "unknown" — User-Agent відсутній, пошкоджений або не розпізнаний.
    #[serde(
        rename = "traffic_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_type: Option<TrafficType>,
    /// IP-адреса отримувача. Присутня лише якщо вдалося її визначити, для статусів "clicked" і "opened".
    #[serde(rename = "ip", default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// ISO 3166-1 alpha-2 код країни, визначений за IP-адресою отримувача. Присутній для статусів "clicked"
    /// і "opened".
    #[serde(
        rename = "country_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_code: Option<String>,
    /// Країна, визначена за IP-адресою отримувача. Присутня для статусів "clicked" і "opened".
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Місто, визначене за IP-адресою отримувача. Присутнє для статусів "clicked" і "opened".
    #[serde(rename = "city", default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// IP-адреса SMTP-сервера, з якого було надіслано лист. Присутня лише для статусів "delivered",
    /// "soft_bounced" і "hard_bounced", якщо доступна.
    #[serde(rename = "sender_ip", default, skip_serializing_if = "Option::is_none")]
    pub sender_ip: Option<String>,
}

/// WebhookGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookGetRequest {
    /// URL webhook.
    #[serde(rename = "url", default)]
    pub url: String,
}

/// WebhookGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookGetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Об'єкт з усіма властивостями webhook.
    #[serde(rename = "object", default, skip_serializing_if = "Option::is_none")]
    pub object: Option<WebhookObject>,
}

/// WebhookListRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookListRequest {
    /// Максимальна кількість webhook, що повертаються за один виклик. Рекомендується 50.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Індекс першого webhook зі списку, що повертається, рахуючи з нуля.
    /// За замовчуванням: `0`.
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// WebhookListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів з даними про webhook.
    #[serde(rename = "objects", default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<WebhookObject>>,
}

/// WebhookDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookDeleteRequest {
    /// URL webhook.
    #[serde(rename = "url", default)]
    pub url: String,
}
