mod inmemory;
mod dynamodb;

pub use self::inmemory::InmemoryEventStore;
pub use self::dynamodb::DynamoDbEventStore;
