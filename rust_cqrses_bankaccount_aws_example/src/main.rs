mod eventstore;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

use rust_cqrses_bankaccount::aggregate::{
    BankAccountId,
    BankAccountName,
};

use rust_cqrses_bankaccount::usecase::command::BankAccountAggregateUseCase;

use eventstore::DynamoDbBankAccountEventStore;

fn main() {
    let region = Region::Custom {
        name: String::from("ap-northeast-1"),
        endpoint: String::from("http://db:8000"),
    };

    let client = DynamoDbClient::new(region);

    let eventstore = Box::new(DynamoDbBankAccountEventStore::new(client));

    let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

    let usecase = BankAccountAggregateUseCase::new(eventstore);

    let result = usecase.open(bank_account_id.clone(), BankAccountName::new(String::from("foo")).unwrap());
    assert!(result.is_ok());
    let result = usecase.get(bank_account_id.clone());
    println!("{:?}", result);

    let result = usecase.update(bank_account_id.clone(), BankAccountName::new(String::from("foo")).unwrap());
    assert!(result.is_ok());
    let result = usecase.get(bank_account_id.clone());
    println!("{:?}", result);

    let result = usecase.deposit(bank_account_id.clone(), 500);
    assert!(result.is_ok());
    let result = usecase.get(bank_account_id.clone());
    println!("{:?}", result);

    let result = usecase.withdraw(bank_account_id.clone(), 300);
    assert!(result.is_ok());
    let result = usecase.get(bank_account_id.clone());
    println!("{:?}", result);

    let result = usecase.close(bank_account_id.clone());
    assert!(result.is_ok());
    let result = usecase.get(bank_account_id.clone());
    println!("{:?}", result);
}
