use std::env;
use failure;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

use rust_cqrses_bankaccount::aggregate::{
    BankAccountId,
    BankAccountName,
};

use rust_cqrses_bankaccount::usecase::command::BankAccountAggregateUseCase;

use rust_cqrses_bankaccount_aws_example::eventstore::DynamoDbBankAccountEventStore;
use rust_cqrses_bankaccount_aws_example::eventpublisher::KafkaBankAccountEventPublisher;

fn main() {
    env_logger::init();

    let config = match Config::from_cmdline() {
        Ok(c) => c,
        Err(err) => {
            println!("{}", err.to_string());
            return;
        },
    };

    let region = Region::Custom {
        name: config.dynamodb_region.clone(),
        endpoint: config.dynamodb_endpoint.clone(),
    };

    let client = DynamoDbClient::new(region);

    let eventpublisher = KafkaBankAccountEventPublisher::new(config.kafka_brokers.clone());

    let eventstore = Box::new(DynamoDbBankAccountEventStore::new(client, eventpublisher));

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

#[derive(Debug)]
struct Config {
    dynamodb_endpoint: String,
    dynamodb_region: String,
    kafka_brokers: Vec<String>,
}

impl Config {
    fn from_cmdline() -> Result<Config, failure::Error> {
        let args: Vec<_> = env::args().collect();
        let mut opts = getopts::Options::new();
        opts.optflag("h", "help", "Print this help screen");
        opts.optopt("", "dynamodb-endpoint", "Dynamodb endpoint", "ENDPOINT");
        opts.optopt("", "dynamodb-region", "Dynamodb region", "REGION");
        opts.optopt("", "kafka-broker", "Specify kafka broker", "HOST");

        let m = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => failure::bail!(e),
        };

        if m.opt_present("help") {
            let brief = format!("{} [options]", args[0]);
            failure::bail!(opts.usage(&brief));
        }

        let dynamodb_endpoint = match m.opt_str("dynamodb-endpoint") {
            Some(s) => s,
            None => failure::bail!(format!("Required option --{} missing", "dynamodb-endpoint")),
        };

        let dynamodb_region = match m.opt_str("dynamodb-region") {
            Some(s) => s,
            None => failure::bail!(format!("Required option --{} missing", "dynamodb-region")),
        };

        let kafka_brokers = m.opt_strs("kafka-broker");
        if kafka_brokers.is_empty() {
            failure::bail!(format!("Required option --{} missing", "kafka-broker"));
        };

        Ok(Config {
            dynamodb_endpoint: dynamodb_endpoint,
            dynamodb_region: dynamodb_region,
            kafka_brokers: kafka_brokers,
        })
    }
}
