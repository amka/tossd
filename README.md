# Tossd - Cервис управления соглашениями

Предназначен для управления соглашениями и их версиями, а так же состояничем принятия пользоваталем.


## Сценарии работы

### Со стороны конечного пользователя

- Получение списка непринятых соглашений
- Получение соглашения с текстом актуальной (последней) версии
- Принятия соглашения в актуальной версии

### Со стороны администратора сервиса

- Создание соглашения
- Добавление версии соглашение
- Изменение соглашения
- Удаление соглашения

## API

### Со стороны конечного пользователя

- `GetUnacceptedAgreementIds(GetUnacceptedRequest)` - Возвращает список идентификаторов соглашений, 
для которых отсутствует флаг "принято" пользователем в рамках указанного провайдера и актуальных версий.
Например: 
  - пользователь ранее принял Соглашение 1 и его Версию 2, актуальную на момент принятия
  - позже администратором была добавлена новая версия Соглашения 1 - 3-я.
  - при следующем вызове этот метод вернёт идентификатор Соглашения 1, т. к. для актуальная версия (номер 3) не принята пользователем.

- `GetAgreement(int agreementId)` - Возвращает соглашение и текст актуальной версии. Актуальной считается версия 
с самым большим порядковым номером.

- `AcceptAgreement(int agreementId, int userId)` - устанавливает флаг "принято пользователем" для актуальной версии.

### Со стороны администратора сервиса

- `CreateAgreement(CreateAgreementRequest) -> AgreementReply` - создаёт новое Соглашение в сервисе и его изначальную Версию за номером 1.

  Если указан список провайдеров - Соглашение будет связано с этими провайдерами.
  
- `CreateVersion(CreateVersionRequst) -> VersionReply` - добавляет к указанному Соглашению новую версию. 
При создании удаляет все записи о принятии Соглашения пользователями, т. к. актуальной становится созданная Версия.

- `UpdateAgreement(UpdateAgreementRequest) - AgreementReply` - обновляет метаданные Соглашения, 
без изменения актуальной Версии. Позволяет изменить заголовки и привязку к провайдерам.

- `DeleteAgreement(DeleteAgreementRequest)` - помечает Соглашение `далённым` - такое Соглашение
не возвращается в списке "непринятых" и используется как исторические данные.

