mod protos;

use log::info;

use std::sync::Arc;

use structopt::StructOpt;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::bank_account::{
    OpenBankAccountRequest,
    UpdateBankAccountRequest,
    DepositBankAccountRequest,
    WithdrawBankAccountRequest,
    CloseBankAccountRequest,
};

use protos::bank_account_grpc::BankAccountServiceClient;

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let args = Args::from_args();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&format!("{}:{}", args.host, args.port));
    let client = BankAccountServiceClient::new(ch);

    match args.cmd {
        Command::Open{ name } => open_bank_account(&client, name),
        Command::Update{ bank_account_id, name } => update_bank_account(&client, bank_account_id, name),
        Command::Deposit{ bank_account_id, deposit } => deposit_bank_account(&client, bank_account_id, deposit),
        Command::Withdraw{ bank_account_id, withdraw } => withdraw_bank_account(&client, bank_account_id, withdraw),
        Command::Close{ bank_account_id } => close_bank_account(&client, bank_account_id),
    };
}

#[derive(StructOpt, Debug)]
#[structopt(name = "grpc_client")]
pub struct Args {
    #[structopt(long, default_value="127.0.0.1")]
    pub host: String,

    #[structopt(long, default_value="8080")]
    pub port: u16,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Open {
        name: String,
    },
    Update {
        bank_account_id: String,
        name: String,
    },
    Deposit {
        bank_account_id: String,
        deposit: i32,
    },
    Withdraw {
        bank_account_id: String,
        withdraw: i32,
    },
    Close {
        bank_account_id: String,
    },
}

fn open_bank_account(client: &BankAccountServiceClient, name: String) {
    let mut req = OpenBankAccountRequest::default();
    req.set_name(name);

    info!("Send request: {:?}", &req);

    let reply = client.open(&req).expect("rpc");

    info!("Response received: {:?}", &reply);
}

fn update_bank_account(client: &BankAccountServiceClient, bank_account_id: String, name: String) {
    let mut req = UpdateBankAccountRequest::default();
    req.set_bank_account_id(bank_account_id);
    req.set_name(name);

    info!("Send request: {:?}", &req);

    let reply = client.update(&req).expect("rpc");

    info!("Response received: {:?}", &reply);
}

fn deposit_bank_account(client: &BankAccountServiceClient, bank_account_id: String, deposit: i32) {
    let mut req = DepositBankAccountRequest::default();
    req.set_bank_account_id(bank_account_id);
    req.set_deposit(deposit);

    info!("Send request: {:?}", &req);

    let reply = client.deposit(&req).expect("rpc");

    info!("Response received: {:?}", &reply);
}

fn withdraw_bank_account(client: &BankAccountServiceClient, bank_account_id: String, withdraw: i32) {
    let mut req = WithdrawBankAccountRequest::default();
    req.set_bank_account_id(bank_account_id);
    req.set_withdraw(withdraw);

    info!("Send request: {:?}", &req);

    let reply = client.withdraw(&req).expect("rpc");

    info!("Response received: {:?}", &reply);
}

fn close_bank_account(client: &BankAccountServiceClient, bank_account_id: String) {
    let mut req = CloseBankAccountRequest::default();
    req.set_bank_account_id(bank_account_id);

    info!("Send request: {:?}", &req);

    let reply = client.close(&req).expect("rpc");

    info!("Response received: {:?}", &reply);
}
