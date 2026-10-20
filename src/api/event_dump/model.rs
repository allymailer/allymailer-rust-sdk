//! Models of the EventDump section.

use serde::{Deserialize, Serialize};

/// EventDumpCreateRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpCreateRequest {
    /// Дата й час у форматі РРРР-ММ-ДД гг:хх:сс, починаючи з якої необхідно отримати події. Дані
    /// зберігаються до 45 днів, залежно від вашого тарифу.
    #[serde(rename = "start_time", default)]
    pub start_time: String,
    /// Дата й час у форматі РРРР-ММ-ДД гг:хх:сс, до якої (не включно) запитуються події.
    #[serde(rename = "end_time", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Максимальна кількість подій, що повертаються, за замовчуванням 50. Якщо це значення перевищує 100
    /// 000, буде створено декілька файлів, кожен з яких містить максимум 100 000 подій.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Якщо у користувача увімкнено проєкти, all_projects=true дозволить отримати дані всіх проєктів.
    /// За замовчуванням: `False`.
    #[serde(
        rename = "all_projects",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_projects: Option<bool>,
    /// Об'єкт із властивостями фільтра дампа подій.
    #[serde(rename = "filter", default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterObject>,
    /// Масив із рядків з переліком назв стовпців, які необхідно отримати у вивантаженні (у порядку
    /// переліку). Якщо 'dump_fields' відсутній або порожній — застосовується набір і порядок полів за
    /// замовчуванням. Припустимі значення (у порядку за замовчуванням): 'project_id', 'event_time',
    /// 'job_id', 'from_email', 'email', 'status', 'delivery_status', 'metadata', 'destination_response',
    /// 'user_agent', 'url', 'ip', 'tags'. З метою більшої конфіденційності поле 'subject' не входить до
    /// набору полів за замовчуванням, але ви можете додати його, вказавши в параметрі 'dump_fields'.
    #[serde(
        rename = "dump_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dump_fields: Option<Vec<String>>,
    /// Дозволяє отримати агреговану статистику за вказаний період. Може набувати одного з двох значень:
    ///
    /// - __day_status__ - отримати агреговану статистику; за такого значення параметр filter має бути відсутнім або порожнім об'єктом (фільтри неприпустимі);
    /// - __порожній рядок__ - значення за замовчуванням, еквівалентне відсутності параметра aggregate, і метод event_dump/create у цьому разі працює як раніше.
    #[serde(rename = "aggregate", default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<String>,
    /// Роздільник полів, за замовчуванням ','. Припустимі значення ',' і ';'.
    #[serde(rename = "delimiter", default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// Формат отримуваного файлу “csv” (за замовчуванням) або “csv_gzip”.
    /// За замовчуванням: `csv`.
    #[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Об'єкт із властивостями фільтра дампа подій.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilterObject {
    /// Ідентифікатор завдання, раніше повернутий методом
    /// [email/send](https://docs.allymailer.com.ua/ua/web-api-ref#email-send).
    #[serde(rename = "job_id", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Статус email. Можна вказати одне з таких значень:
    ///
    /// __accepted__ — повідомлення прийнято, але ще не надіслано.
    ///
    /// __sent__ — повідомлення надіслано, але ще не доставлено.
    ///
    /// __delivered__ — повідомлення доставлено.
    ///
    /// __opened__ — повідомлення доставлено й зареєстровано його прочитання.
    ///
    /// __clicked__ — повідомлення доставлено, прочитано, було зареєстровано перехід за одним з посилань у
    /// листі.
    ///
    /// __unsubscribed__ — повідомлення було доставлено отримувачу й прочитано ним, але користувач
    /// відписався за посиланням у листі.
    ///
    /// __subscribed__ — повідомлення було доставлено отримувачу й прочитано ним, але користувач відписався,
    /// а потім знову підписався за посиланням у листі.
    ///
    /// __soft_bounced__ — тимчасова недоставка повідомлення. AllyMailer продовжує спроби доставки протягом
    /// 2 діб.
    ///
    /// __hard_bounced__ — доставка не вдалася, спроб доставки більше не буде.
    ///
    /// __spam__ — повідомлення доставлено, але позначено як “спам” отримувачем.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::common::model::EmailStatus>,
    /// Внутрішній детальний статус доставки AllyMailer. Можна вказати декілька статусів через кому. Ось
    /// деякі з можливих статусів:
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
    /// Адреса отримувача
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Адреса відправника
    #[serde(
        rename = "email_from",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_from: Option<String>,
    /// Домен отримувача
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Ідентифікатор розсилки, невід'ємне десяткове ціле або UUID до 128 біт, передане в метаданих у полі з
    /// іменем "campaign_id" (ім'я можна налаштувати через техпідтримку).
    #[serde(
        rename = "campaign_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub campaign_id: Option<String>,
}

/// EventDumpCreateResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpCreateResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Ідентифікатор дампа. Використовується для виклику методу
    /// [event-dump/get](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-get).
    #[serde(rename = "dump_id", default, skip_serializing_if = "Option::is_none")]
    pub dump_id: Option<String>,
}

/// EventDumpGetRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpGetRequest {
    /// Ідентифікатор дампа, отриманий методом
    /// [event-dump/create](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create).
    #[serde(rename = "dump_id", default)]
    pub dump_id: String,
}

/// EventDumpGetResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpGetResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Об'єкт із властивостями дампа подій.
    #[serde(
        rename = "event_dump",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_dump: Option<EventDumpObject>,
}

/// Об'єкт із властивостями дампа подій.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpObject {
    /// Ідентифікатор дампа, отриманий методом
    /// [event-dump/create](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create).
    #[serde(rename = "dump_id", default, skip_serializing_if = "Option::is_none")]
    pub dump_id: Option<String>,
    /// Статус задачі. Одне з чотирьох значень:
    ///
    /// - __queued__ - очікує в черзі, обробка ще не починалася;
    /// - __in_process__ - триває обробка;
    /// - __ready__ - обробку завершено, файли доступні для завантаження;
    /// - __failed__ - обробку перервано з помилкою.
    #[serde(
        rename = "dump_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dump_status: Option<String>,
    /// Масив об'єктів, кожен з яких позначає готовий до завантаження файл. Навіть якщо
    /// "dump_status"="in_process", уже можна завантажувати ті файли, які вказані в "files". У разі
    /// відсутності подій за заданими параметрами буде повернуто порожній масив.
    #[serde(rename = "files", default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<EventDumpFilesObject>>,
}

/// Об'єкт із властивостями файлу дампа подій.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpFilesObject {
    /// Унікальний URL для завантаження файлу з розширенням .csv або .csv.gz, залежно від запитаного
    /// формату.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Розмір файлу.
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// EventDumpListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів, що описують дампи подій. Кожен об'єкт містить ті самі властивості, що й об'єкт
    /// 'event_dump', який повертає метод
    /// [event-dump/get](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-get).
    #[serde(
        rename = "event_dumps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_dumps: Option<Vec<EventDumpObject>>,
}

/// EventDumpDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventDumpDeleteRequest {
    /// Ідентифікатор дампа, отриманий методом
    /// [event-dump/create](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump-create).
    #[serde(rename = "dump_id", default)]
    pub dump_id: String,
}
