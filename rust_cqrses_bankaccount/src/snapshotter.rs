use std::fmt;
use chrono::Local;
use failure::{Fail, Context, Backtrace};

use super::eventsourcing::{Snapshot, EventStoreError};
use super::aggregate::{BankAccountId, BankAccountAggregate, Error as BankAccountError};

use super::BankAccountEventStore;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Fail)]
pub enum ErrorKind {
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

pub struct BankAccountAggregateSnapshotter {
    eventstore: Box<BankAccountEventStore>,
}

impl BankAccountAggregateSnapshotter {
    pub fn new(eventstore: Box<BankAccountEventStore>) -> Self {
        Self {
            eventstore: eventstore,
        }
    }

    pub fn take_snapshot(&self, bank_account_id: BankAccountId) -> Result<(), Error> {
        let stream_id = BankAccountAggregate::stream_id(&bank_account_id);
        self.eventstore.event_stream_since(stream_id, 1)
            .map_err(|err| Error::from(err))
            .and_then(|stream| {
                let aggregate = BankAccountAggregate::new();
                let history = stream.events().clone();
                BankAccountAggregate::load_from_history(&aggregate, history, stream.version())
                    .map_err(|err| Error::from(err))
            })
            .and_then(|aggregate| {
                let snapshot = Snapshot::new(
                    BankAccountAggregate::stream_id(aggregate.id()),
                    aggregate.version(),
                    aggregate.state().as_ref().unwrap().clone(),
                    Local::now(),
                    );
                self.eventstore.record_snapshot(snapshot)
                    .map_err(|err| Error::from(err))
            })
            .map(|_| ())
    }
}
