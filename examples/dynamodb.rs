use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

use rust_cqrses_bankaccount::aggregate::{
    BankAccountCommand,
    BankAccountId,
    BankAccountName,
    BankAccountAggregate,
    BankAccountAggregateRepository,
};

use rust_cqrses_bankaccount::infra::eventstore::DynamoDbEventStore;

use rust_cqrses_bankaccount::infra::repository::{
    EventStoreBankAccountAggregateRepository,
};

fn main() {
    let region: Region = Region::Custom {
        name: String::from("ap-northeast-1"),
        endpoint: String::from("http://db:8000"),
    };
    let client = DynamoDbClient::new(region);

    let event_store = DynamoDbEventStore::new(client);
    let repository = EventStoreBankAccountAggregateRepository::new(event_store);

    let mut aggregate = BankAccountAggregate::new();

    let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

    aggregate.handle_command(BankAccountCommand::Open {
        bank_account_id: bank_account_id.clone(),
        name: BankAccountName::new(String::from("foo")).unwrap(),
    }).unwrap();

    aggregate.handle_command(BankAccountCommand::Update {
        bank_account_id: bank_account_id.clone(),
        name: BankAccountName::new(String::from("bar")).unwrap(),
    }).unwrap();

    aggregate.handle_command(BankAccountCommand::Deposit {
        bank_account_id: bank_account_id.clone(),
        deposit: 500,
    }).unwrap();

    aggregate.handle_command(BankAccountCommand::Withdraw {
        bank_account_id: bank_account_id.clone(),
        withdraw: 300,
    }).unwrap();

    aggregate.handle_command(BankAccountCommand::Close {
        bank_account_id: bank_account_id.clone(),
    }).unwrap();

    repository.save(aggregate).unwrap();
}
