use std::fmt;
use failure::{Fail, Context, Backtrace};

use super::super::aggregate::{
    BankAccountCommand,
    BankAccountId,
    BankAccountName,
    BankAccountAggregate,
    Error as BankAccountError,
};

use super::super::eventsourcing::{EventStoreError, EventStoreErrorKind};

use super::super::BankAccountEventStore;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "BankAccount does not exits: {:?}", _0)]
    BankAccountNotFound(BankAccountId),

    #[fail(display = "Event store error")]
    EventStoreError,

    #[fail(display = "Bank account error")]
    BankAccountError,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self{ inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Self { inner: inner }
    }
}

impl From<BankAccountError> for Error {
    fn from(error: BankAccountError) -> Self {
        Self { inner: error.context(ErrorKind::BankAccountError) }
    }
}

impl From<EventStoreError> for Error {
    fn from(error: EventStoreError) -> Self {
        Self { inner: error.context(ErrorKind::EventStoreError) }
    }
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

    pub fn get(&self, bank_account_id: BankAccountId) -> Result<BankAccountAggregate, Error> {
        match self.load_aggregate(&bank_account_id) {
            Some(aggregate) => Ok(aggregate),
            None => Err(ErrorKind::BankAccountNotFound(bank_account_id.clone()))?,
        }
    }

    pub fn open(&self, bank_account_id: BankAccountId, name: BankAccountName)
        -> Result<(), Error> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Open {
            bank_account_id: bank_account_id.clone(),
            name: name.clone(),
        })
    }

    pub fn update(&self, bank_account_id: BankAccountId, name: BankAccountName)
        -> Result<(), Error> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Update {
            bank_account_id: bank_account_id.clone(),
            name: name.clone(),
        })
    }

    pub fn deposit(&self, bank_account_id: BankAccountId, deposit: i32)
        -> Result<(), Error> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Deposit {
            bank_account_id: bank_account_id.clone(),
            deposit: deposit,
        })
    }

    pub fn withdraw(&self, bank_account_id: BankAccountId, withdraw: i32)
        -> Result<(), Error> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Withdraw {
            bank_account_id: bank_account_id.clone(),
            withdraw: withdraw,
        })
    }

    pub fn close(&self, bank_account_id: BankAccountId)
        -> Result<(), Error> {
        self.handle_command(bank_account_id.clone(), BankAccountCommand::Close {
            bank_account_id: bank_account_id.clone(),
        })
    }

    fn handle_command(&self, id: BankAccountId, command: BankAccountCommand)
        -> Result<(), Error> {
        let mut aggregate = match self.load_aggregate(&id) {
            Some(aggregate) => aggregate,
            None => BankAccountAggregate::new(),
        };

        BankAccountAggregate::handle_command(&aggregate, command)
            .map_err(|err| Error::from(err))
            .and_then(|events| {
                for event in events.iter() {
                    aggregate = match BankAccountAggregate::apply_event(&aggregate, event.clone()) {
                        Ok(aggregate) => aggregate,
                        Err(err) => {
                            return Err(err)?;
                        },
                    };
                }
                Ok((aggregate, events))
            })
            .and_then(|(aggregate, events)| {
                let stream_id = BankAccountAggregate::stream_id(aggregate.id());
                let stream_version = aggregate.version() + 1;

                self.eventstore
                    .append_event_stream(stream_id, stream_version, events.clone())
                    .map_err(|err| Error::from(err))
                    .map(|_| events)
            })
            .map(|_| ())
    }

    fn load_aggregate(&self, id: &BankAccountId) -> Option<BankAccountAggregate> {
        match self.eventstore.read_snapshot(BankAccountAggregate::stream_id(id)).unwrap() {
            Some(snapshot) => {
                let stream_id = BankAccountAggregate::stream_id(id);
                let stream_version = snapshot.stream_version() + 1;
                match self.eventstore.event_stream_since(stream_id, stream_version) {
                    Ok(stream) => {
                        let aggregate = BankAccountAggregate::load_from_snapshot(snapshot);
                        let history = stream.events().clone();
                        Some(BankAccountAggregate::load_from_history(&aggregate, history, stream.version()).unwrap())
                    },
                    Err(e) => match e.kind() {
                        EventStoreErrorKind::NoEventStreamError(_, _) => Some(BankAccountAggregate::load_from_snapshot(snapshot)),
                        _ => panic!(e.to_string()),
                    },
                }
            },
            None => {
                let stream_id = BankAccountAggregate::stream_id(id);
                match self.eventstore.event_stream_since(stream_id, 1) {
                    Ok(stream) => {
                        let aggregate = BankAccountAggregate::new();
                        let history = stream.events().clone();
                        Some(BankAccountAggregate::load_from_history(&aggregate, history, stream.version()).unwrap())
                    },
                    Err(e) => match e.kind() {
                        EventStoreErrorKind::NoEventStreamError(_, _) => None,
                        _ => panic!(e.to_string()),
                    },

                }
            },
        }
    }
}
