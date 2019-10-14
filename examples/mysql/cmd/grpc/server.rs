mod protos;

use std::sync::Arc;
use log::{error, info, debug};
use futures::Future;
use chan::chan_select;
use chan_signal::{kill_this, Signal};
use structopt::StructOpt;

use grpcio::{
    RpcContext,
    UnarySink,
    RpcStatus,
    RpcStatusCode,
    EnvBuilder,
    ServerBuilder,
};

use protos::bank_account::{
    OpenBankAccountRequest,
    OpenBankAccountResponse,
    UpdateBankAccountRequest,
    UpdateBankAccountResponse,
    DepositBankAccountRequest,
    DepositBankAccountResponse,
    WithdrawBankAccountRequest,
    WithdrawBankAccountResponse,
    CloseBankAccountRequest,
    CloseBankAccountResponse,
};

use protos::bank_account_grpc::{BankAccountService, create_bank_account_service};

use rust_cqrses_bankaccount::aggregate::{BankAccountId, BankAccountName};
use rust_cqrses_bankaccount::usecase::command::BankAccountAggregateUseCase;

use rust_cqrses_bankaccount_mysql_example::Config;
use rust_cqrses_bankaccount_mysql_example::constants;
use rust_cqrses_bankaccount_mysql_example::db;
use rust_cqrses_bankaccount_mysql_example::eventstore::MysqlBankAccountEventStore;
use rust_cqrses_bankaccount_mysql_example::eventpublisher::KafkaBankAccountEventPublisher;

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let config = envy::from_env::<Config>().unwrap();

    let args = Args::from_args();

    let pool = db::init_database_pool(&config.database_url);

    let eventpublisher = KafkaBankAccountEventPublisher::new(config.kafka_brokers.clone(), String::from(constants::TOPIC));

    let eventstore = Box::new(MysqlBankAccountEventStore::new(pool, eventpublisher));

    let usecase = Arc::new(BankAccountAggregateUseCase::new(eventstore));

    let env = Arc::new(EnvBuilder::new().build());

    let mut sv = ServerBuilder::new(env)
        .register_service(create_bank_account_service(Server::new(usecase)))
        .bind(args.host, args.port)
        .build()
        .expect("fail build server");
    sv.start();

    for &(ref host, port) in sv.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }

    // Signal gets a value when the OS sent a INT or TERM or USR1 signal.
    let sig = chan_signal::notify(&[Signal::INT, Signal::TERM, Signal::USR1]);
    chan_select! {
        sig.recv() -> signal => {
            debug!("receive signal={:?}", signal);
            kill_this(Signal::TERM);
        }
    }

    let _ = sv.shutdown().wait();
}

#[derive(StructOpt, Debug)]
#[structopt(name = "grpc_server")]
pub struct Args {
    #[structopt(long, default_value="127.0.0.1")]
    pub host: String,

    #[structopt(long, default_value="8080")]
    pub port: u16,
}

#[derive(Clone)]
pub struct Server {
    usecase: Arc<BankAccountAggregateUseCase>,
}

impl Server {
    pub fn new(usecase: Arc<BankAccountAggregateUseCase>) -> Self {
        Self {
            usecase: usecase,
        }
    }
}

impl BankAccountService for Server {
    fn open(&mut self, ctx: RpcContext, req: OpenBankAccountRequest, sink: UnarySink<OpenBankAccountResponse>) {
        let bank_account_id = BankAccountId::new(uuid::Uuid::new_v4().to_hyphenated().to_string()).unwrap();
        let name = match BankAccountName::new(String::from(req.get_name())) {
            Ok(n) => n,
            Err(err) => {
                let f = sink.fail(RpcStatus::new(RpcStatusCode::InvalidArgument, Some(err.to_string())))
                    .map_err(|err| error!("failed to fail response {:?}", err));
                ctx.spawn(f);
                return;
            },
        };

        let usecase = self.usecase.clone();

        let f = match usecase.open(bank_account_id.clone(), name) {
            Ok(_) => {
                let mut resp = OpenBankAccountResponse::new();
                resp.set_bank_account_id(bank_account_id.value().to_string());
                sink.success(resp)
            },
            Err(err) => {
                error!("An error occurred when open bank account: {:?}", err);
                sink.fail(RpcStatus::new(RpcStatusCode::Internal, None))
            },
        };

        ctx.spawn(f.map_err(|err| error!("failed to fail response {:?}", err)));
    }

    fn update(&mut self, ctx: RpcContext, req: UpdateBankAccountRequest, sink: UnarySink<UpdateBankAccountResponse>) {
        let bank_account_id = match BankAccountId::new(String::from(req.get_bank_account_id())) {
            Ok(id) => id,
            Err(err) => {
                let f = sink.fail(RpcStatus::new(RpcStatusCode::InvalidArgument, Some(err.to_string())))
                    .map_err(|err| error!("failed to fail response {:?}", err));
                ctx.spawn(f);
                return;
            },
        };

        let name = match BankAccountName::new(String::from(req.get_name())) {
            Ok(n) => n,
            Err(err) => {
                let f = sink.fail(RpcStatus::new(RpcStatusCode::InvalidArgument, Some(err.to_string())))
                    .map_err(|err| error!("failed to fail response {:?}", err));
                ctx.spawn(f);
                return;
            },
        };

        let usecase = self.usecase.clone();

        let f = match usecase.update(bank_account_id.clone(), name) {
            Ok(_) => {
                sink.success(UpdateBankAccountResponse::new())
            },
            Err(err) => {
                error!("An error occurred when update bank account: {:?}", err);
                sink.fail(RpcStatus::new(RpcStatusCode::Internal, None))
            },
        };

        ctx.spawn(f.map_err(|err| error!("failed to fail response {:?}", err)));
    }

    fn deposit(&mut self, ctx: RpcContext, req: DepositBankAccountRequest, sink: UnarySink<DepositBankAccountResponse>) {
        let bank_account_id = match BankAccountId::new(String::from(req.get_bank_account_id())) {
            Ok(id) => id,
            Err(err) => {
                let f = sink.fail(RpcStatus::new(RpcStatusCode::InvalidArgument, Some(err.to_string())))
                    .map_err(|err| error!("failed to fail response {:?}", err));
                ctx.spawn(f);
                return;
            },
        };

        let deposit = req.get_deposit();

        let usecase = self.usecase.clone();

        let f = match usecase.deposit(bank_account_id.clone(), deposit) {
            Ok(_) => {
                sink.success(DepositBankAccountResponse::new())
            },
            Err(err) => {
                error!("An error occurred when update bank account: {:?}", err);
                sink.fail(RpcStatus::new(RpcStatusCode::Internal, None))
            },
        };

        ctx.spawn(f.map_err(|err| error!("failed to fail response {:?}", err)));
    }

    fn withdraw(&mut self, ctx: RpcContext, req: WithdrawBankAccountRequest, sink: UnarySink<WithdrawBankAccountResponse>) {
        let bank_account_id = match BankAccountId::new(String::from(req.get_bank_account_id())) {
            Ok(id) => id,
            Err(err) => {
                let f = sink.fail(RpcStatus::new(RpcStatusCode::InvalidArgument, Some(err.to_string())))
                    .map_err(|err| error!("failed to fail response {:?}", err));
                ctx.spawn(f);
                return;
            },
        };

        let withdraw = req.get_withdraw();

        let usecase = self.usecase.clone();

        let f = match usecase.withdraw(bank_account_id.clone(), withdraw) {
            Ok(_) => {
                sink.success(WithdrawBankAccountResponse::new())
            },
            Err(err) => {
                error!("An error occurred when update bank account: {:?}", err);
                sink.fail(RpcStatus::new(RpcStatusCode::Internal, None))
            },
        };

        ctx.spawn(f.map_err(|err| error!("failed to fail response {:?}", err)));
    }

    fn close(&mut self, ctx: RpcContext, req: CloseBankAccountRequest, sink: UnarySink<CloseBankAccountResponse>) {
        let bank_account_id = match BankAccountId::new(String::from(req.get_bank_account_id())) {
            Ok(id) => id,
            Err(err) => {
                let f = sink.fail(RpcStatus::new(RpcStatusCode::InvalidArgument, Some(err.to_string())))
                    .map_err(|err| error!("failed to fail response {:?}", err));
                ctx.spawn(f);
                return;
            },
        };

        let usecase = self.usecase.clone();

        let f = match usecase.close(bank_account_id.clone()) {
            Ok(_) => {
                sink.success(CloseBankAccountResponse::new())
            },
            Err(err) => {
                error!("An error occurred when update bank account: {:?}", err);
                sink.fail(RpcStatus::new(RpcStatusCode::Internal, None))
            },
        };

        ctx.spawn(f.map_err(|err| error!("failed to fail response {:?}", err)));
    }
}
