use failure::Fail;

use super::super::aggregate::{
    BankAccountEvent,
    BankAccountCommand,
    BankAccountId,
    BankAccountName,
    BankAccountAggregate,
    BankAccountError,
};

use super::super::eventsourcing::{EventStreamId, EventStream, EventStore, EventStoreError};

type BankAccountEventStore = dyn EventStore<Event = BankAccountEvent, EventStream = EventStream<BankAccountEvent>>;

#[derive(Debug, Fail)]
pub enum BankAccountAggregateUseCaseError {
    #[fail(display = "BankAccount does not exits: {:?}", _0)]
    BankAccountNotFound(BankAccountId),

    #[fail(display = "Event store error: {:?}", _0)]
    EventStoreError(#[fail(cause)] EventStoreError),

    #[fail(display = "Event store error: {:?}", _0)]
    BankAccountError(#[fail(cause)] BankAccountError),
}

pub struct BankAccountAggregateUseCase {
    eventstore: Box<BankAccountEventStore>,
}

impl BankAccountAggregateUseCase {
    pub fn new(eventstore: Box<BankAccountEventStore>) -> Self {
        Self {
            eventstore: eventstore,
        }
    }

    pub fn get(&self, bank_account_id: BankAccountId) -> Result<BankAccountAggregate, BankAccountAggregateUseCaseError> {
        match self.load_aggregate(&bank_account_id) {
            Some(aggregate) => Ok(aggregate),
            None => Err(BankAccountAggregateUseCaseError::BankAccountNotFound(bank_account_id.clone())),
        }
    }

    pub fn open(&self, bank_account_id: BankAccountId, name: BankAccountName)
        -> Result<(), BankAccountAggregateUseCaseError> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Open {
            bank_account_id: bank_account_id.clone(),
            name: name.clone(),
        })
    }

    pub fn update(&self, bank_account_id: BankAccountId, name: BankAccountName)
        -> Result<(), BankAccountAggregateUseCaseError> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Update {
            bank_account_id: bank_account_id.clone(),
            name: name.clone(),
        })
    }

    pub fn deposit(&self, bank_account_id: BankAccountId, deposit: i32)
        -> Result<(), BankAccountAggregateUseCaseError> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Deposit {
            bank_account_id: bank_account_id.clone(),
            deposit: deposit,
        })
    }

    pub fn withdraw(&self, bank_account_id: BankAccountId, withdraw: i32)
        -> Result<(), BankAccountAggregateUseCaseError> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Withdraw {
            bank_account_id: bank_account_id.clone(),
            withdraw: withdraw,
        })
    }

    pub fn close(&self, bank_account_id: BankAccountId)
        -> Result<(), BankAccountAggregateUseCaseError> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Close {
            bank_account_id: bank_account_id.clone(),
        })
    }

    fn handle_command(&self, id: BankAccountId, command: BankAccountCommand)
        -> Result<(), BankAccountAggregateUseCaseError> {
        let mut aggregate = match self.load_aggregate(&id) {
            Some(aggregate) => aggregate,
            None => BankAccountAggregate::new(),
        };

        BankAccountAggregate::handle_command(&aggregate, command)
            .map_err(|e| BankAccountAggregateUseCaseError::BankAccountError(e))
            .and_then(|events| {
                for event in events.iter() {
                    aggregate = match BankAccountAggregate::apply_event(&aggregate, event.clone()) {
                        Ok(aggregate) => aggregate,
                        Err(e) => {
                            return Err(BankAccountAggregateUseCaseError::BankAccountError(e));
                        },
                    };
                }

                let stream_name = self.stream_name(aggregate.id());
                let stream_version = aggregate.version() + 1;
                let stream_id =  EventStreamId::new(stream_name, stream_version);

                match self.eventstore.save(stream_id, events) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(BankAccountAggregateUseCaseError::EventStoreError(e)),
                }
            })
    }

    fn load_aggregate(&self, id: &BankAccountId) -> Option<BankAccountAggregate> {
        let stream_name = self.stream_name(id);
        let stream_id =  EventStreamId::new(stream_name, 1);
        let result = &self.eventstore.event_stream_since(&stream_id);

        match result {
            Ok(stream) => {
                let events = stream.events().clone();
                Some(BankAccountAggregate::load_from_events(events, stream.version()).unwrap())
            },
            Err(e) => match e {
                EventStoreError::NoEventStreamError(_, _) => None,
                _ => panic!(e.to_string()),
            },
        }
    }

    fn stream_name(&self, id: &BankAccountId) -> String {
        format!("bank_account:{}", id)
    }
}
