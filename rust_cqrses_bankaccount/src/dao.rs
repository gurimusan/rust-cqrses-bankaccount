use chrono::{Local, DateTime};

pub struct BankAccountRM {
    pub bank_account_id: String,
    pub name: String,
    pub is_closed: bool,
    pub balance: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub version: u64,
}

pub trait BankAccountDao {
    fn find(&self, bank_account_id: String) -> Option<BankAccountRM>;

    fn insert(&self, model: BankAccountRM);

    fn update(&self, model: BankAccountRM);
}
