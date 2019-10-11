use chrono::{Local, DateTime};

pub struct BankAccountRecord {
    pub id: String,
    pub name: String,
    pub is_closed: bool,
    pub balance: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub version: u64,
}

pub trait BankAccountDao {
    fn find(&self, id: String) -> Option<BankAccountRecord>;

    fn insert(&self, record: BankAccountRecord);

    fn update(&self, record: BankAccountRecord);
}
