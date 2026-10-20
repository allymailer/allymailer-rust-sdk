//! Models of the Email section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// EmailSendRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailSendRequest {
    /// Об'єкт, що містить усі властивості повідомлення, яке надсилається.
    #[serde(rename = "message", default)]
    pub message: MessageObject,
}

/// Об'єкт, що містить усі властивості повідомлення, яке надсилається.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageObject {
    /// Масив об'єктів з email-адресами, підстановками та метаданими отримувачів.
    #[serde(rename = "recipients", default)]
    pub recipients: Vec<MessageObjectRecipientsItem>,
    /// Унікальний ідентифікатор шаблону, створеного раніше методом
    /// [template/set](https://docs.allymailer.com.ua/ua/web-api-ref#template-set). Якщо вказано, то поля
    /// шаблону підставляються замість пропущених полів email/send. Наприклад, якщо в email/send не вказано
    /// body — береться тіло листа із шаблону, а якщо не вказано subject — береться тема із шаблону.
    #[serde(
        rename = "template_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<String>,
    /// Масив із рядків, до 100 символів у кожному. Максимум 4 рядки в масиві, рядки мають бути унікальними.
    /// Для кожного облікового запису/проєкту дозволено не більше ніж 10 000 міток, у разі перевищення метод
    /// email/send повертатиме помилку. Мітки можуть використовуватися для категоризації листів за обраними
    /// вами критеріями. Мітки передаються методами event-dump.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Пропустити чи не пропускати додавання стандартного блока з посиланням відписки до HTML-частини
    /// листа. 1=пропустити, 0=додати, за замовчуванням 0. Для использования skip_unsubscribe=1 вам надо
    /// [попросить техподдержку](https://cp.allymailer.com/ru/support) включить такую возможность.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "skip_unsubscribe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_unsubscribe: Option<i64>,
    /// Заголовок для вибору мови посилання та сторінки відписки. Припустимі значення "be", "de", "en",
    /// "es", "fr", "it", "pl", "pt", "ru", "ua", "kz".
    #[serde(
        rename = "global_language",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_language: Option<String>,
    /// Параметр для вибору [шаблонізатора](https://docs.allymailer.com.ua/en/template-engines) — або
    /// "simple", або "velocity", або "liquid", або "none". За замовчуванням — "simple".
    #[serde(
        rename = "template_engine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_engine: Option<crate::common::model::TemplateEngine>,
    /// Об'єкт для передачі глобальних підстановок (наприклад, назва компанії). Якщо назви змінних
    /// повторюються в об'єкті користувацьких підстановок "substitutions", значення змінних будуть взяті з
    /// об'єкта "substitutions". Підстановки працюють у параметрах:
    ///
    /// - body.html
    /// - body.plaintext
    /// - body.amp
    /// - subject
    /// - from_name
    /// - options.unsubscribe_url
    ///
    /// Назви змінних можуть містити літери латинського алфавіту, цифри, символ "_". Першим символом може
    /// бути лише літера, використання пробілів не допускається.
    #[serde(
        rename = "global_substitutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_substitutions: Option<HashMap<String, String>>,
    /// Об'єкт для передачі метаданих, спільних для всіх отримувачів, у форматі: "key": "value". Максимальна
    /// кількість ключів: 10; максимальна довжина ключа: 64 символи; максимальна довжина значення ключа:
    /// 1024 символи. Метадані передаються під час виклику
    /// [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook). Для групування листів в одну
    /// розсилку може використовуватися системний ключ "campaign_id" зі значенням у вигляді невід'ємного
    /// десяткового цілого до 128 біт або UUID вигляду "c7703772-453e-4c77-a9ae-9a1b94051b5a". Неправильні
    /// значення будуть прирівняні до '0'
    #[serde(
        rename = "global_metadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_metadata: Option<HashMap<String, String>>,
    /// Об'єкт, що містить у собі html-, plaintext- і amp-частини листа. Або html-, або plaintext-частина
    /// має бути присутня обов'язково.
    #[serde(rename = "body", default)]
    pub body: crate::common::model::BodyObject,
    /// Тема листа.
    #[serde(rename = "subject", default)]
    pub subject: String,
    /// Email-адреса відправника. Обов'язковий лише за порожнього template_id.
    #[serde(
        rename = "from_email",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_email: Option<String>,
    /// Ім'я відправника.
    #[serde(rename = "from_name", default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Необов'язкова email-адреса для відповідей (на випадок, якщо відрізняється від адреси відправника)
    #[serde(rename = "reply_to", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Необов'язкове ім'я для відповідей (якщо вказано email reply_to і ви хочете, щоб відображався не лише
    /// цей email, але й ім'я)
    #[serde(
        rename = "reply_to_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_name: Option<String>,
    /// 1 = відстеження переходів за посиланнями увімкнено (значення за замовчуванням), 0=вимкнено. Для
    /// використання track_links = 0 вам потрібно [попросити
    /// техпідтримку](https://cp.allymailer.com/ua/support) увімкнути таку можливість.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "track_links",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_links: Option<i64>,
    /// 1 = відстеження прочитань листа увімкнено (значення за замовчуванням), 0 = вимкнено. Для
    /// використання track_read = 0 вам потрібно [попросити
    /// техпідтримку](https://cp.allymailer.com/ua/support) увімкнути таку можливість.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "track_read",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub track_read: Option<i64>,
    /// (0 або 1, за замовчуванням 0) Значення 1=ігнорувати список глобальної недоступності, під час
    /// надсилання не враховуватимуться дані про недоступність адрес або скарги, отримані під час надсилання
    /// іншими користувачами AllyMailer. Для деяких системних записів може бути заборонено
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "bypass_global",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_global: Option<i64>,
    /// (0 або 1, за замовчуванням 0) Значення 1=ігнорувати список недоступних адрес поточного облікового
    /// запису/проєкту. Указання 1 можливе лише за одночасного вказання bypass_global=1.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "bypass_unavailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_unavailable: Option<i64>,
    /// (0 або 1, за замовчуванням 0) Значення 1=ігнорувати список відписаних від розсилок поточного
    /// облікового запису/проєкту. Указання 1 можливе лише за одночасного вказання bypass_global=1. Параметр
    /// доступний лише тим користувачам, яким [техпідтримка](https://cp.allymailer.com/ua/support) схвалила
    /// вимкнення посилання відписки.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "bypass_unsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_unsubscribed: Option<i64>,
    /// (0 або 1, за замовчуванням 0) Значення 1=ігнорувати список тих, хто поскаржився на обліковий
    /// запис/проєкт. Указання 1 можливе лише за одночасного вказання bypass_global=1. Параметр доступний
    /// лише тим користувачам, яким [техпідтримка](https://cp.allymailer.com/ua/support) схвалила вимкнення
    /// посилання відписки.
    /// Допустимі значення: `0`, `1`.
    #[serde(
        rename = "bypass_complained",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_complained: Option<i64>,
    /// Рядок до 64 символів завдовжки з унікальним ключем запиту. Може використовуватися для запобігання
    /// випадковому дублюванню листів. У разі повторного надсилання запиту з тим самим ключем протягом
    /// хвилини запит буде відхилено. Може створюватися автоматично для кожного листа; щоб увімкнути
    /// автоматичну генерацію ключа, зв'яжіться з [техпідтримкою](https://cp.allymailer.com/ua/support).
    #[serde(
        rename = "idempotence_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub idempotence_key: Option<String>,
    /// Об'єкт, що містить додаткові заголовки листа, максимум 50 заголовків. Префікс “X-” в імені заголовка
    /// обов'язковий, заголовки без нього будуть проігноровані, наприклад X-ALLYMAILER-Global-Language,
    /// X-ALLYMAILER-Template-Engine. Стандартні заголовки "To", "CC", "BCC" передаються без префікса "X-",
    /// але обробляються спеціальним чином, тому мають низку обмежень, докладніше про це можна прочитати
    /// [тут](/cc-and-bcc). Також якщо [техпідтримка](https://cp.allymailer.com/ua/support) схвалила вам
    /// вимкнення посилання відписки, то ви також можете передавати заголовки List-Unsubscribe,
    /// List-Subscribe, List-Help, List-Owner, List-Archive, In-Reply-To і References.
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::common::model::HeadersObject>,
    /// Необов'язковий масив вкладень.
    #[serde(
        rename = "attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments: Option<Vec<crate::common::model::AttachmentsArrayItem>>,
    /// Масив для передачі inline-вкладень, наприклад, для включення зображень у лист замість завантаження
    /// їх за посиланням.
    #[serde(
        rename = "inline_attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_attachments: Option<Vec<crate::common::model::InlineAttachmentsArrayItem>>,
    /// Додаткові опції.
    #[serde(rename = "options", default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MessageObjectOptions>,
}

/// MessageObjectRecipientsItem
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageObjectRecipientsItem {
    /// Email-адреса отримувача
    #[serde(rename = "email", default)]
    pub email: String,
    /// Об'єкт, що описує підстановки для конкретного отримувача (наприклад, ім'я, товар для показу саме
    /// цьому отримувачу — див. [Шаблонізатори](https://docs.allymailer.com.ua/en/template-engines)).
    /// Підстановки працюють у параметрах:
    ///
    /// - body.html
    /// - body.plaintext
    /// - body.amp
    /// - subject
    /// - from_name
    /// - headers["List-Unsubscribe"]
    /// - options.unsubscribe_url
    ///
    /// Назви змінних можуть містити літери латинського алфавіту, цифри, символ "_". Першим символом може
    /// бути лише літера, використання пробілів не допускається. Є спеціальна змінна "to_name", що дозволяє
    /// задати ім'я отримувача. У разі підстановки туди "Ім'я Прізвище" SMTP-заголовок “To” виглядатиме як
    /// "Ім'я Прізвище <email@example.com>". Довжина змінної "to_name" обмежена 78 символами.
    #[serde(
        rename = "substitutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub substitutions: Option<HashMap<String, String>>,
    /// Об'єкт для передачі метаданих, специфічних для окремого отримувача, де "key" — рядок до 64 символів
    /// завдовжки, "value" — рядок до 1024 символів завдовжки, максимальна кількість ключів — 10. Метадані
    /// передаються під час виклику [webhook](https://docs.allymailer.com.ua/ua/web-api-ref#webhook) і
    /// методів [event-dump](https://docs.allymailer.com.ua/ua/web-api-ref#event-dump). Для групування
    /// листів в одну розсилку може використовуватися системний ключ "campaign_id" зі значенням у вигляді
    /// невід'ємного десяткового цілого або UUID до 128 біт.
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Додаткові опції.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageObjectOptions {
    /// Дата й час у форматі "РРРР-ММ-ДД гг:хх:сс" у часовому поясі UTC. Дозволяє запланувати час надсилання
    /// на майбутнє, у межах 24 годин від поточного часу.
    #[serde(rename = "send_at", default, skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
    /// Користувацьке посилання відписки. Докладніше [тут](https://docs.allymailer.com.ua/unsubscribe-link).
    #[serde(
        rename = "unsubscribe_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_url: Option<String>,
    /// Ідентифікатор backend-домену, через який надсилається повідомлення. Якщо відсутній — надсилання буде
    /// від імені backend-домену, призначеного вашому обліковому запису/проєкту за замовчуванням. Ви можете
    /// придбати один або декілька виділених IP-адрес і використовувати цю опцію для вибору.
    #[serde(
        rename = "custom_backend_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_backend_id: Option<i64>,
    /// Ідентифікатор пулу SMTP, через який надсилається повідомлення. Зазвичай немає необхідності вказувати
    /// пул явно, оскільки він обирається автоматично або як пул за замовчуванням для вашого облікового
    /// запису/проєкту, або відповідно до переданого параметра custom_backend_id.
    #[serde(
        rename = "smtp_pool_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_pool_id: Option<String>,
}

/// EmailSendResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailSendResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Ідентифікатор завдання з надсилання, може виявитися корисним під час з'ясування причин збоїв.
    #[serde(rename = "job_id", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Масив email-адрес, успішно прийнятих до надсилання.
    #[serde(rename = "emails", default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Об'єкт із email-адресами, не прийнятими до надсилання. Email представлено іменем властивості, а його
    /// статус — значенням властивості, наприклад: {"email1@gmail.com": "temporary_unavailable"}. Можливі
    /// статуси адрес, на які не вдалося здійснити надсилання:
    ///
    /// - __unsubscribed__ — зазначену адресу відписано;
    /// - __invalid__ — адреса не існує або введена некоректно;
    /// - __duplicate__ — адреса повторюється в запиті, повторне надсилання на той самий email запобігнуто;
    /// - __temporary_unavailable__ — адреса тимчасово недоступна. Це означає, що протягом наступних трьох діб надсилання на цю адресу повертатиме помилку. Email може бути тимчасово недоступним з різних причин, наприклад:
    ///
    /// - попереднє надсилання було відхилено сервером отримувача як спам;
    /// - поштова скринька отримувача переповнена або не використовується;
    /// - домен не приймає пошту через неправильне налаштування на боці отримувача або з інших причин;
    /// - сервер відправника було відхилено через блеклистинг;
    ///
    /// - __permanent_unavailable__ — адреса перманентно недоступна через багаторазові недоставки;
    /// - __complained__ — в одному з попередніх листів адресат натиснув “Це спам”;
    /// - __blocked__ — надсилання на цю адресу заборонено адміністрацією AllyMailer.
    ///
    /// У майбутньому можуть з'явитися й інші статуси.
    #[serde(
        rename = "failed_emails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_emails: Option<HashMap<String, String>>,
}

/// EmailSubscribeRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailSubscribeRequest {
    /// Адреса відправника на підтвердженому домені. (Також підтримується іменування поля як
    /// "email_address_from").
    #[serde(rename = "from_email", default)]
    pub from_email: String,
    /// Ім'я відправника. (Також підтримується іменування поля як "name_from").
    #[serde(rename = "from_name", default)]
    pub from_name: String,
    /// Адреса отримувача. (Також підтримується іменування поля як "email_address_to").
    #[serde(rename = "to_email", default)]
    pub to_email: String,
}
