use chrono::{Local, DateTime};
use super::dao::{BankAccountRM, BankAccountRMDao};
use super::aggregate::{BankAccountEvent, BankAccountId, BankAccountName};

pub struct BankAccountProjector {
    dao: Box<dyn BankAccountRMDao>,
}

impl BankAccountProjector {
    pub fn new(dao: Box<dyn BankAccountRMDao>) -> Self {
        Self { dao }
    }

    pub fn project(&self, event: BankAccountEvent) {
        match event {
            BankAccountEvent::Opened{ bank_account_id, name, occurred_at } => self.create(bank_account_id, name, occurred_at),
            BankAccountEvent::Updated{ bank_account_id, name, occurred_at } => self.update(bank_account_id, name, occurred_at),
            BankAccountEvent::Deposited{ bank_account_id, deposit, occurred_at } => self.deposit(bank_account_id, deposit, occurred_at),
            BankAccountEvent::Withdrawn{ bank_account_id, withdraw, occurred_at } => self.withdraw(bank_account_id, withdraw, occurred_at),
            BankAccountEvent::Closed{ bank_account_id, occurred_at } => self.close(bank_account_id, occurred_at),
        };
    }

    fn create(&self, id: BankAccountId, name: BankAccountName, occurred_at: DateTime<Local>) {
        self.dao.insert(BankAccountRM {
            bank_account_id: id.to_string(),
            name: name.to_string(),
            is_closed: false,
            balance: 0,
            created_at: occurred_at.clone(),
            updated_at: occurred_at.clone(),
            version: 1,
        });
    }

    fn update(&self, id: BankAccountId, name: BankAccountName, occurred_at: DateTime<Local>) {
        let mut record = self.dao.find(id.to_string()).unwrap();
        record.name = name.to_string();
        record.updated_at = occurred_at;
        record.version = record.version + 1;
        self.dao.update(record);
    }

    fn deposit(&self, id: BankAccountId, deposit: i32, occurred_at: DateTime<Local>) {
        let mut record = self.dao.find(id.to_string()).unwrap();
        record.balance = record.balance + deposit;
        record.updated_at = occurred_at;
        record.version = record.version + 1;
        self.dao.update(record);
    }

    fn withdraw(&self, id: BankAccountId, withdraw: i32, occurred_at: DateTime<Local>) {
        let mut record = self.dao.find(id.to_string()).unwrap();
        record.balance = record.balance - withdraw;
        record.updated_at = occurred_at;
        record.version = record.version + 1;
        self.dao.update(record);
    }

    fn close(&self, id: BankAccountId, occurred_at: DateTime<Local>) {
        let mut record = self.dao.find(id.to_string()).unwrap();
        record.is_closed = true;
        record.updated_at = occurred_at;
        record.version = record.version + 1;
        self.dao.update(record);
    }
}
