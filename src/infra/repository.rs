use failure::Fail;

use serde_json::error::Error as SerdeJsonError;

use super::super::aggregate::{
    BankAccountError,
    BankAccountId,
    BankAccountAggregate,
    BankAccountAggregateRepository,
};
use super::super::eventstore::{EventStreamId, Event, EventStoreError, EventStore};

#[derive(Debug, Fail)]
pub enum EventStoreBankAccountAggregateRepositoryError {
    #[fail(display = "EventStore error: {:?}", _0)]
    EventStoreError(EventStoreError),

    #[fail(display = "BankAccount error: {:?}", _0)]
    BankAccountError(BankAccountError),

    #[fail(display = "Serd json error: {:?}", _0)]
    SerdeJsonError(SerdeJsonError),

    #[fail(display = "Invalid event type: {:?}", _0)]
    InvalidEventType(String),
}

pub struct EventStoreBankAccountAggregateRepository<ES: EventStore> {
    eventstore: ES,
}

impl<ES> EventStoreBankAccountAggregateRepository<ES>
where ES: EventStore {
    pub fn new(eventstore: ES) -> Self {
        Self {
            eventstore: eventstore,
        }
    }

    fn stream_name(&self, id: BankAccountId) -> String {
        format!("bank_account:{}", id)
    }
}

impl<ES> BankAccountAggregateRepository for EventStoreBankAccountAggregateRepository<ES>
where ES: EventStore {
    type Error = EventStoreBankAccountAggregateRepositoryError;

    fn save(&self, aggregate: BankAccountAggregate) -> Result<(), Self::Error> {
        let stream_name = self.stream_name(aggregate.id());
        let stream_version = aggregate.version() + 1;
        let stream_id =  EventStreamId::new(stream_name, stream_version);
        let events: Vec<Event> = aggregate.events().iter().map(|event| {
            event.clone().into()
        })
        .collect();
        self.eventstore.save(stream_id, events)
            .map_err(|err| Self::Error::EventStoreError(err))
    }

    fn bank_account_of_id(&self, id: BankAccountId) -> Result<BankAccountAggregate, Self::Error> {
        let stream_name = self.stream_name(id);
        let stream_id =  EventStreamId::new(stream_name, 1);
        match self.eventstore.event_stream_since(&stream_id) {
            Ok(stream) => {
                let result: Result<Vec<_>, _> = stream.events().iter()
                    .map(|event| {
                        serde_json::from_str(event.event_body())
                            .map_err(|e| Self::Error::SerdeJsonError(e))
                    })
                    .collect();
                match result {
                    Ok(events) => match BankAccountAggregate::load_from_events(events, stream.version()) {
                        Ok(aggregate) => Ok(aggregate),
                        Err(e) => Err(Self::Error::BankAccountError(e)),
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(Self::Error::EventStoreError(e)),
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

    use super::super::super::aggregate::{
        BankAccountEvent,
        BankAccountCommand,
        BankAccountId,
        BankAccountName,
        BankAccount,
        BankAccountAggregate,
        BankAccountAggregateRepository
    };
    use super::super::super::eventstore::{EventStreamId, Event, EventStream, EventStoreError, EventStore};
    use super::EventStoreBankAccountAggregateRepository;

    struct MockedEventStore;

    impl EventStore for MockedEventStore {
        fn save(&self, _id: EventStreamId, _events: Vec<Event>) -> Result<(), EventStoreError> {
            Ok(())
        }

        fn event_stream_since(&self, id: &EventStreamId) -> Result<EventStream, EventStoreError> {
            let domain_events = vec![
                BankAccountEvent::Opened {
                    bank_account_id: BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap(),
                    name: BankAccountName::new(String::from("foo")).unwrap(),
                    occurred_at: Local::now(),
                },
            ];
            Ok(EventStream::new(domain_events.iter().map(|event| event.clone().into()).collect(), 1))
        }
    }

    #[test]
    fn test_save_aggregate() {
        let eventstore = MockedEventStore{};

        let repo = EventStoreBankAccountAggregateRepository::new(eventstore);

        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let mut aggregate = BankAccountAggregate::load(BankAccount::new(
                bank_account_id.clone(),
                BankAccountName::new(String::from("foo")).unwrap(),
                false,
                0,
                Local::now(),
                Local::now(),
                ), 1);

        aggregate.handle_command(BankAccountCommand::Update {
            bank_account_id: bank_account_id.clone(),
            name: BankAccountName::new(String::from("bar")).unwrap(),
        }).unwrap();

        aggregate.handle_command(BankAccountCommand::Deposit {
            bank_account_id: bank_account_id.clone(),
            deposit: 500,
        }).unwrap();

        let result = repo.save(aggregate);

        assert!(result.is_ok());
    }

    #[test]
    fn test_bank_account_of_id() {
        let eventstore = MockedEventStore{};

        let repo = EventStoreBankAccountAggregateRepository::new(eventstore);

        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let result = repo.bank_account_of_id(bank_account_id);

        assert!(result.is_ok());
    }
}
