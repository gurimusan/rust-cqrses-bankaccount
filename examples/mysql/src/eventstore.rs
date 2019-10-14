use chrono::{Local, NaiveDateTime, TimeZone};

use rust_cqrses_bankaccount::eventsourcing::{
    EventStream,
    Snapshot,
    EventStoreError,
    EventStoreErrorKind,
    EventStore,
    EventPublisher,
};
use rust_cqrses_bankaccount::aggregate::{BankAccountEvent, BankAccount};

use diesel::prelude::*;
use super::schema::{tbl_event_store, tbl_snapshot};
use super::db::{Conn, Pool};
use super::eventpublisher::KafkaBankAccountEventPublisher;

pub struct MysqlBankAccountEventStore {
    pool: Pool,
    publisher: KafkaBankAccountEventPublisher,
}

impl MysqlBankAccountEventStore {
    pub fn new(pool: Pool, publisher: KafkaBankAccountEventPublisher) -> Self {
        Self {
            pool,
            publisher,
        }
    }

    pub fn get_conn(&self) -> Result<Conn, r2d2::Error> {
        self.pool.get()
    }
}

impl EventStore for MysqlBankAccountEventStore {
    type Event = BankAccountEvent;
    type EventStream = EventStream<Self::Event>;
    type SnapshotData = BankAccount;

    fn append_event_stream(&self, stream_id: String, stream_version: u64, events: Vec<Self::Event>)
        -> Result<(), EventStoreError> {

        let conn = self.get_conn().unwrap();

        conn.transaction::<_, diesel::result::Error, _>(|| {
            let mut i: u64 = 0;
            let mut stream_version = stream_version;

            for event in &events {
                stream_version = stream_version + i;

                let new_event = NewEventRecord {
                    event_type: event.event_type(),
                    event_body: &serde_json::to_string(&event).unwrap(),
                    stream_id: &stream_id,
                    stream_version: stream_version,
                    event_occurred_at: event.occurred_at().naive_local(),
                };

                diesel::insert_into(tbl_event_store::table)
                    .values(&new_event)
                    .execute(&conn)?;

                i = i + 1;
            }

            Ok(events)
        }).map_err(|err| {
            EventStoreError::from(EventStoreErrorKind::AppendEventStreamError(err.to_string()))
        })
        .and_then(|events| {
            for event in events {
                self.publisher.publish(event.clone());
            }
            Ok(())
        })
    }

    fn event_stream_since(&self, stream_id: String, stream_version: u64)
        -> Result<Self::EventStream, EventStoreError> {
        let conn = self.get_conn().unwrap();

        tbl_event_store::table
            .filter(tbl_event_store::stream_id.eq(stream_id.clone()))
            .filter(tbl_event_store::stream_version.ge(stream_version))
            .order(tbl_event_store::stream_version.asc())
            .load::<EventRecord>(&conn)
            .map_err(|err| EventStoreError::from(EventStoreErrorKind::QueryError(err.to_string())))
            .and_then(|event_records| {
                if event_records.len() == 0 {
                    return Err(EventStoreErrorKind::NoEventStreamError(stream_id.clone(), stream_version))?;
                }

                let events = event_records.iter()
                    .map(|event_record| serde_json::from_str(&event_record.event_body).unwrap())
                    .collect();
                let version = event_records.last().unwrap().stream_version;
                Ok(EventStream::new(events, version))
            })
    }

    fn record_snapshot(&self, snapshot: Snapshot<Self::SnapshotData>)
        -> Result<(), EventStoreError> {
        let conn = self.get_conn().unwrap();

        conn.transaction::<_, diesel::result::Error, _>(|| {
            let new_snapshot = NewSnapshotRecord {
                stream_id: snapshot.stream_id(),
                stream_version: snapshot.stream_version(),
                data: &serde_json::to_string(snapshot.snapshot()).unwrap(),
                created_at: snapshot.created_at().naive_local(),
            };

            diesel::replace_into(tbl_snapshot::table)
                .values(&new_snapshot)
                .execute(&conn)?;

            Ok(())
        }).map_err(|err| {
            EventStoreError::from(EventStoreErrorKind::AppendEventStreamError(err.to_string()))
        })
    }

    fn read_snapshot(&self, stream_id: String)
        -> Result<Option<Snapshot<Self::SnapshotData>>, EventStoreError> {
        let conn = self.get_conn().unwrap();

        tbl_snapshot::table
            .filter(tbl_snapshot::stream_id.eq(stream_id.clone()))
            .first::<SnapshotRecord>(&conn)
            .optional()
            .map_err(|err| EventStoreError::from(EventStoreErrorKind::QueryError(err.to_string())))
            .and_then(|result| {
                match result {
                    Some(record) => {
                        Ok(Some(Snapshot::new(
                            record.stream_id,
                            record.stream_version,
                            serde_json::from_str(&record.data).unwrap(),
                            Local.from_local_datetime(&record.created_at).unwrap(),
                            )))
                    },
                    None => Ok(None),
                }
            })
    }
}

#[derive(Insertable)]
#[table_name = "tbl_event_store"]
struct NewEventRecord<'a> {
    event_type: &'a str,
    event_body: &'a str,
    stream_id: &'a str,
    stream_version: u64,
    event_occurred_at: NaiveDateTime,
}

#[derive(Debug, Queryable)]
struct EventRecord {
    event_id: u64,
    event_body: String,
    event_type: String,
    stream_id: String,
    stream_version: u64,
    event_occurred_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tbl_snapshot"]
struct NewSnapshotRecord<'a> {
    stream_id: &'a str,
    stream_version: u64,
    data: &'a str,
    created_at: NaiveDateTime,
}

#[derive(Queryable)]
struct SnapshotRecord {
    stream_id: String,
    stream_version: u64,
    data: String,
    created_at: NaiveDateTime,
}
