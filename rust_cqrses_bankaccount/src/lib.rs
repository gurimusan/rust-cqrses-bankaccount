pub mod eventsourcing;
pub mod aggregate;
pub mod usecase;
pub mod inmemory_eventstore;
pub mod snapshotter;
pub mod dao;
pub mod projector;

use aggregate::{BankAccountEvent, BankAccount};
use eventsourcing::{EventStream, EventStore, EventPublisher};

pub type BankAccountEventStore = dyn EventStore<Event = BankAccountEvent,
                                                EventStream = EventStream<BankAccountEvent>,
                                                SnapshotData = BankAccount>;

pub type BankAccountEventPublisher = dyn EventPublisher<Event = BankAccountEvent>;
