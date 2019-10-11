use std::fmt;
use chrono::{Local, DateTime};
use uuid::Uuid;
use failure::{Fail, Context, Backtrace};
use serde::{Serialize, Deserialize};
use super::eventsourcing::Snapshot;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Invalid bank account id: {:?}", _0)]
    InvalidBankAccountId(String),

    #[fail(display = "Invalid bank account name: {:?}", _0)]
    InvalidBankAccountName(String),

    #[fail(display = "State is not yet opended")]
    NotYetOpened,

    #[fail(display = "State is already opended: {:?}", _0)]
    AlreadyOpened(BankAccountId),

    #[fail(display = "State is already closed: {:?}", _0)]
    AlreadyClosed(BankAccountId),

    #[fail(display = "A deposited money amount 0 is illegal: id = {:?}, money = {:?}", _0, _1)]
    DepositZero(BankAccountId, i32),

    #[fail(display = "Forbidden that deposit amount to negative: id = {:?}, money = {:?}", _0, _1)]
    NegativeBalance(BankAccountId, i32),

    #[fail(display = "Invalid state: {:?}", _0)]
    InvalidState(BankAccountId),
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
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BankAccountEvent {
    Opened {
        bank_account_id: BankAccountId,
        name: BankAccountName,
        occurred_at: DateTime<Local>,
    },
    Updated {
        bank_account_id: BankAccountId,
        name: BankAccountName,
        occurred_at: DateTime<Local>,
    },
    Deposited {
        bank_account_id: BankAccountId,
        deposit: i32,
        occurred_at: DateTime<Local>,
    },
    Withdrawn {
        bank_account_id: BankAccountId,
        withdraw: i32,
        occurred_at: DateTime<Local>,
    },
    Closed {
        bank_account_id: BankAccountId,
        occurred_at: DateTime<Local>,
    },
}

impl BankAccountEvent {
    pub fn event_type(&self) -> &str {
        match self {
            Self::Opened {bank_account_id: _, name: _, occurred_at: _} => "BankAccountOpened",
            Self::Updated {bank_account_id: _, name: _, occurred_at: _} => "BankAccountUpdated",
            Self::Deposited {bank_account_id: _, deposit: _, occurred_at: _} => "BankAccountDeposited",
            Self::Withdrawn {bank_account_id: _, withdraw: _, occurred_at: _} => "BankAccountWithdrawn",
            Self::Closed {bank_account_id: _, occurred_at: _} => "BankAccountClosed",
        }
    }

    pub fn occurred_at(&self) -> DateTime<Local> {
        match self {
            Self::Opened {bank_account_id: _, name: _, occurred_at} => occurred_at.clone(),
            Self::Updated {bank_account_id: _, name: _, occurred_at} => occurred_at.clone(),
            Self::Deposited {bank_account_id: _, deposit: _, occurred_at} => occurred_at.clone(),
            Self::Withdrawn {bank_account_id: _, withdraw: _, occurred_at} => occurred_at.clone(),
            Self::Closed {bank_account_id: _, occurred_at} => occurred_at.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BankAccountCommand {
    Open {
        bank_account_id: BankAccountId,
        name: BankAccountName,
    },
    Update {
        bank_account_id: BankAccountId,
        name: BankAccountName,
    },
    Deposit {
        bank_account_id: BankAccountId,
        deposit: i32,
    },
    Withdraw {
        bank_account_id: BankAccountId,
        withdraw: i32,
    },
    Close {
        bank_account_id: BankAccountId,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BankAccountId {
    value: Uuid,
}

impl BankAccountId {
    pub fn new(value: String) -> Result<Self, Error> {
        match Uuid::parse_str(&value) {
            Ok(uuid) => Ok(Self { value: uuid }),
            Err(_) => Err(ErrorKind::InvalidBankAccountId(value))?,
        }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

impl fmt::Display for BankAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  self.value().to_hyphenated().to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BankAccountName {
    value: String,
}

impl BankAccountName {
    pub fn new(value: String) -> Result<Self, Error> {
        if !value.is_empty() && value.len() < 255 {
            Ok(Self { value: value })
        } else {
            Err(ErrorKind::InvalidBankAccountName(value))?
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for BankAccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  self.value())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankAccount {
    id: BankAccountId,
    name: BankAccountName,
    is_closed: bool,
    balance: i32,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl BankAccount {
    pub fn new(
        id: BankAccountId,
        name: BankAccountName,
        is_closed: bool,
        balance: i32,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
        ) -> Self {
        Self {
            id: id,
            name: name,
            is_closed: is_closed,
            balance: balance,
            created_at: created_at,
            updated_at: updated_at,
        }
    }

    pub fn id(&self) -> &BankAccountId {
        &self.id
    }

    pub fn name(&self) -> &BankAccountName {
        &self.name
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn balance(&self) -> i32 {
        self.balance
    }

    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Local> {
        &self.updated_at
    }

    pub fn with_name(&self, name: BankAccountName, occurred_at: DateTime<Local>)
        -> Result<Self, Error> {
        if self.is_closed {
            Err(ErrorKind::AlreadyClosed(self.id.clone()))?
        } else {
            Ok(Self {
                name: name,
                updated_at: occurred_at,
                .. self.clone()
            })
        }
    }

    pub fn deposit(&self, deposit: i32, occurred_at: DateTime<Local>)
        -> Result<Self, Error> {
        if self.is_closed {
            Err(ErrorKind::AlreadyClosed(self.id.clone()))?
        } else if deposit == 0 {
            Err(ErrorKind::DepositZero(self.id.clone(), deposit))?
        } else if (self.balance + deposit) < 0 {
            Err(ErrorKind::NegativeBalance(self.id.clone(), deposit))?
        } else {
            Ok(Self {
                balance: self.balance + deposit,
                updated_at: occurred_at,
                .. self.clone()
            })
        }
    }

    pub fn withdraw(&self, withdraw: i32, occurred_at: DateTime<Local>) -> Result<Self, Error> {
        if self.is_closed {
            Err(ErrorKind::AlreadyClosed(self.id.clone()))?
        } else if withdraw == 0 {
            Err(ErrorKind::DepositZero(self.id.clone(), withdraw))?
        } else if (self.balance - withdraw) < 0 {
            Err(ErrorKind::NegativeBalance(self.id.clone(), withdraw))?
        } else {
            Ok(Self {
                balance: self.balance - withdraw,
                updated_at: occurred_at,
                .. self.clone()
            })
        }
    }

    pub fn close(&self, occurred_at: DateTime<Local>) -> Result<Self, Error> {
        if self.is_closed {
            Err(ErrorKind::AlreadyClosed(self.id.clone()))?
        } else {
            Ok(Self {
                is_closed: true,
                updated_at: occurred_at,
                .. self.clone()
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct BankAccountAggregate {
    state: Option<BankAccount>,
    version: u64,
}

impl BankAccountAggregate {
    pub fn new() -> Self {
        Self {
            state: None,
            version: 0,
        }
    }

    pub fn stream_id(bank_account_id: &BankAccountId) -> String {
        format!("bank_account:{}", bank_account_id)
    }

    pub fn load(bank_account: BankAccount, version: u64) -> Self {
        Self {
            state: Some(bank_account),
            version: version,
        }
    }

    pub fn load_from_snapshot(snapshot: Snapshot<BankAccount>) -> Self {
        Self::load(snapshot.snapshot().clone(), snapshot.stream_version())
    }

    pub fn load_from_history(aggregate: &Self, history: Vec<BankAccountEvent>, version: u64)
        -> Result<Self, Error> {
        let mut aggregate = aggregate.clone();
        for event in history {
            aggregate = match BankAccountAggregate::apply_event(&aggregate, event) {
                Ok(aggregate) => aggregate,
                Err(e) => return Err(e),
            }
        }
        aggregate.set_version(version);
        Ok(aggregate)
    }

    pub fn handle_command(aggregate: &Self, command: BankAccountCommand)
        -> Result<Vec<BankAccountEvent>, Error> {
        match command {
            BankAccountCommand::Open{ bank_account_id, name } => {
                match aggregate.state() {
                    Some(_) => Err(ErrorKind::AlreadyOpened(bank_account_id))?,
                    None => {
                        Ok(vec![
                            BankAccountEvent::Opened {
                                bank_account_id: bank_account_id,
                                name: name,
                                occurred_at: Local::now(),
                            }
                        ])
                    },
                }
            }
            BankAccountCommand::Update{ bank_account_id, name } => {
                if aggregate.equals_id(&bank_account_id) {
                    Ok(vec![
                        BankAccountEvent::Updated {
                            bank_account_id: bank_account_id,
                            name: name,
                            occurred_at: Local::now(),
                        }
                    ])
                } else {
                    Err(ErrorKind::InvalidState(bank_account_id))?
                }
            },
            BankAccountCommand::Deposit{ bank_account_id, deposit } => {
                if aggregate.equals_id(&bank_account_id) {
                    Ok(vec![
                        BankAccountEvent::Deposited {
                            bank_account_id: bank_account_id,
                            deposit: deposit,
                            occurred_at: Local::now(),
                        }
                    ])
                } else {
                    Err(ErrorKind::InvalidState(bank_account_id))?
                }
            },
            BankAccountCommand::Withdraw{ bank_account_id, withdraw } => {
                if aggregate.equals_id(&bank_account_id) {
                    Ok(vec![
                        BankAccountEvent::Withdrawn {
                            bank_account_id: bank_account_id,
                            withdraw: withdraw,
                            occurred_at: Local::now(),
                        }
                    ])
                } else {
                    Err(ErrorKind::InvalidState(bank_account_id))?
                }
            },
            BankAccountCommand::Close{ bank_account_id } => {
                if aggregate.equals_id(&bank_account_id) {
                    Ok(vec![
                        BankAccountEvent::Closed {
                            bank_account_id: bank_account_id,
                            occurred_at: Local::now(),
                        }
                    ])
                } else {
                    Err(ErrorKind::InvalidState(bank_account_id))?
                }
            },
        }
    }

    pub fn apply_event(aggregate: &Self, event: BankAccountEvent)
        -> Result<Self, Error> {
        match event {
            BankAccountEvent::Opened{ bank_account_id, name, occurred_at } => {
                match aggregate.state() {
                    Some(_) => Err(ErrorKind::AlreadyOpened(bank_account_id))?,
                    None => {
                        Ok(Self {
                            state: Some(BankAccount::new(
                                bank_account_id.clone(),
                                name.clone(),
                                false,
                                0,
                                occurred_at.clone(),
                                occurred_at.clone(),
                                )),
                            version: 0,
                        })
                    },
                }
            },
            BankAccountEvent::Updated{ bank_account_id: _, name, occurred_at } => {
                aggregate.state().as_ref().unwrap().with_name(name.clone(), occurred_at.clone())
                    .and_then(|new_state| {
                        Ok(Self {
                            state: Some(new_state),
                            version: aggregate.version(),
                        })
                    })
            },
            BankAccountEvent::Deposited{ bank_account_id: _, deposit, occurred_at } => {
                aggregate.state().as_ref().unwrap().deposit(deposit, occurred_at.clone())
                    .and_then(|new_state| {
                        Ok(Self {
                            state: Some(new_state),
                            version: aggregate.version(),
                        })
                    })
            },
            BankAccountEvent::Withdrawn{ bank_account_id: _, withdraw, occurred_at } => {
                aggregate.state().as_ref().unwrap().withdraw(withdraw, occurred_at.clone())
                    .and_then(|new_state| {
                        Ok(Self {
                            state: Some(new_state),
                            version: aggregate.version(),
                        })
                    })
            },
            BankAccountEvent::Closed{ bank_account_id: _, occurred_at } => {
                aggregate.state().as_ref().unwrap().close(occurred_at.clone())
                    .and_then(|new_state| {
                        Ok(Self {
                            state: Some(new_state),
                            version: aggregate.version(),
                        })
                    })
            },
        }
    }

    pub fn id(&self) -> &BankAccountId {
        self.state.as_ref().unwrap().id()
    }

    pub fn state(&self) -> &Option<BankAccount> {
        &self.state
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn set_version(&mut self, version: u64) {
        self.version = version;
    }

    pub fn increment_version(&mut self) {
        self.version += 1;
    }

    pub fn equals_id(&self, bank_account_id: &BankAccountId) -> bool {
        match &self.state {
            Some(ba) => ba.id() == bank_account_id,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;
    use chrono::Local;
    use uuid::Uuid;

    use super::ErrorKind;
    use super::BankAccountCommand;
    use super::BankAccountId;
    use super::BankAccountName;
    use super::BankAccount;
    use super::BankAccountAggregate;

    fn create_bank_account(is_closed: bool, balance: i32) -> BankAccount {
        BankAccount {
            id: BankAccountId { value: Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap() },
            name: BankAccountName { value: String::from("foo") },
            is_closed: is_closed,
            balance: balance,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }

    #[test]
    fn test_new_bank_account_id() {
        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8"));
        assert!(bank_account_id.is_ok());
        assert_eq!(bank_account_id.unwrap().to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");

        match BankAccountId::new(String::from("!")) {
            Err(err) => match err.kind() {
                ErrorKind::InvalidBankAccountId(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_new_bank_account_name() {
        let bank_account_name = BankAccountName::new(String::from("foo"));
        assert!(bank_account_name.is_ok());
        assert_eq!(bank_account_name.unwrap().value(), "foo");

        let str255 = std::iter::repeat(())
            .map(|()| thread_rng().sample(Alphanumeric)).take(255).collect();
        match BankAccountName::new(str255) {
            Err(err) => match err.kind() {
                ErrorKind::InvalidBankAccountName(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_bank_account_with_name() {
        let bank_account = create_bank_account(false, 0);
        match bank_account.with_name(BankAccountName::new(String::from("bar")).unwrap(), Local::now()) {
            Ok(ba) => assert_eq!(ba.name().value(), "bar"),
            _ => assert!(false),
        };

        let bank_account = create_bank_account(true, 0);
        match bank_account.with_name(BankAccountName::new(String::from("bar")).unwrap(), Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::AlreadyClosed(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_bank_account_deposit() {
        let bank_account = create_bank_account(false, 0);
        match bank_account.deposit(500, Local::now()) {
            Ok(ba) => assert_eq!(ba.balance(), 500),
            _ => assert!(false),
        };

        let bank_account = create_bank_account(true, 0);
        match bank_account.deposit(500, Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::AlreadyClosed(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };

        let bank_account = create_bank_account(false, 0);
        match bank_account.deposit(0, Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::DepositZero(_, _) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };

        let bank_account = create_bank_account(false, 0);
        match bank_account.deposit(-500, Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::NegativeBalance(_, _) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_bank_account_withdraw() {
        let bank_account = create_bank_account(false, 1000);
        match bank_account.withdraw(500, Local::now()) {
            Ok(ba) => assert_eq!(ba.balance(), 500),
            _ => assert!(false),
        };

        let bank_account = create_bank_account(true, 1000);
        match bank_account.withdraw(500, Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::AlreadyClosed(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };

        let bank_account = create_bank_account(false, 1000);
        match bank_account.withdraw(0, Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::DepositZero(_, _) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };

        let bank_account = create_bank_account(false, 1000);
        match bank_account.withdraw(1100, Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::NegativeBalance(_, _) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_bank_account_close() {
        let bank_account = create_bank_account(false, 0);
        match bank_account.close(Local::now()) {
            Ok(ba) => assert!(ba.is_closed()),
            _ => assert!(false),
        };

        let bank_account = create_bank_account(true, 0);
        match bank_account.close(Local::now()) {
            Err(err) => match err.kind() {
                ErrorKind::AlreadyClosed(_) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_aggregate_handle_open_bank_account_command() {
        let aggregate = BankAccountAggregate::new();

        let result = BankAccountAggregate::handle_command(&aggregate, BankAccountCommand::Open {
            bank_account_id: BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap(),
            name: BankAccountName::new(String::from("foo")).unwrap(),
        });
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        let result = BankAccountAggregate::apply_event(&aggregate, events[0].clone());
        let aggregate = result.unwrap();
        assert!(aggregate.state.is_some());

        let ba = aggregate.state.unwrap();
        assert_eq!(ba.id().value().to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(ba.name().value(), "foo");
        assert_eq!(ba.is_closed(), false);
        assert_eq!(ba.balance(), 0);
    }

    #[test]
    fn test_aggregate_handle_update_bank_account_command() {
        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let aggregate = BankAccountAggregate::load(BankAccount::new(
                bank_account_id.clone(),
                BankAccountName::new(String::from("foo")).unwrap(),
                false,
                0,
                Local::now(),
                Local::now(),
                ), 1);

        let result = BankAccountAggregate::handle_command(&aggregate, BankAccountCommand::Update {
            bank_account_id: bank_account_id.clone(),
            name: BankAccountName::new(String::from("bar")).unwrap(),
        });
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        let result = BankAccountAggregate::apply_event(&aggregate, events[0].clone());
        let aggregate = result.unwrap();
        assert!(aggregate.state.is_some());

        let ba = aggregate.state.unwrap();
        assert_eq!(ba.id().value().to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(ba.name().value(), "bar");
        assert_eq!(ba.is_closed(), false);
        assert_eq!(ba.balance(), 0);
    }

    #[test]
    fn test_aggregate_handle_deposit_bank_account_command() {
        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let aggregate = BankAccountAggregate::load(BankAccount::new(
                bank_account_id.clone(),
                BankAccountName::new(String::from("foo")).unwrap(),
                false,
                0,
                Local::now(),
                Local::now(),
                ), 1);

        let result = BankAccountAggregate::handle_command(&aggregate, BankAccountCommand::Deposit {
            bank_account_id: bank_account_id.clone(),
            deposit: 500,
        });
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        let result = BankAccountAggregate::apply_event(&aggregate, events[0].clone());
        let aggregate = result.unwrap();
        assert!(aggregate.state.is_some());

        let ba = aggregate.state.unwrap();
        assert_eq!(ba.id().value().to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(ba.name().value(), "foo");
        assert_eq!(ba.is_closed(), false);
        assert_eq!(ba.balance(), 500);
    }

    #[test]
    fn test_aggregate_handle_withdraw_bank_account_command() {
        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let aggregate = BankAccountAggregate::load(BankAccount::new(
                bank_account_id.clone(),
                BankAccountName::new(String::from("foo")).unwrap(),
                false,
                500,
                Local::now(),
                Local::now(),
                ), 1);

        let result = BankAccountAggregate::handle_command(&aggregate, BankAccountCommand::Withdraw {
            bank_account_id: bank_account_id.clone(),
            withdraw: 300,
        });
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        let result = BankAccountAggregate::apply_event(&aggregate, events[0].clone());
        let aggregate = result.unwrap();
        assert!(aggregate.state.is_some());

        let ba = aggregate.state.unwrap();
        assert_eq!(ba.id().value().to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(ba.name().value(), "foo");
        assert_eq!(ba.is_closed(), false);
        assert_eq!(ba.balance(), 200);
    }

    #[test]
    fn test_aggregate_handle_close_bank_account_command() {
        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let aggregate = BankAccountAggregate::load(BankAccount::new(
                bank_account_id.clone(),
                BankAccountName::new(String::from("foo")).unwrap(),
                false,
                0,
                Local::now(),
                Local::now(),
                ), 1);

        let result = BankAccountAggregate::handle_command(&aggregate, BankAccountCommand::Close {
            bank_account_id: bank_account_id.clone(),
        });
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        let result = BankAccountAggregate::apply_event(&aggregate, events[0].clone());
        let aggregate = result.unwrap();
        assert!(aggregate.state.is_some());

        let ba = aggregate.state.unwrap();
        assert_eq!(ba.id().value().to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(ba.name().value(), "foo");
        assert_eq!(ba.is_closed(), true);
        assert_eq!(ba.balance(), 0);
    }
}
