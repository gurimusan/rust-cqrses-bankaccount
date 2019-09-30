use chrono::{Local, DateTime};
use diesel::r2d2::ConnectionManager;
use diesel::mysql::MysqlConnection;

use rust_cqrses_bankaccount::dao::{BankAccountRecord, BankAccountDao};
use schema::*;

pub type Conn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Insertable, Queryable)]
#[table_name = "bank_accounts"]
pub struct BankAccount {
    pub id: String,
    pub name: String,
    pub is_closed: bool,
    pub balance: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub version: u64,
}

pub struct MysqlBankAccountDao {
    conn: Rc<Conn>,
}

impl BankAccountDao for MysqlBankAccountDao {
    fn find(&self, id: String) -> Option<BankAccountRecord> {
        bank_accounts::table
            .filter(bank_accounts::id.eq(id))
            .first::<BankAccountRecord>(&**self.conn)
            .optional()
    }

    fn insert(&self, record: BankAccountRecord) {
    }

    fn update(&self, record: BankAccountRecord) {
    }
}

#[derive(Insertable)]
#[table_name = "bank_accounts"]
struct NewBankAccount<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub is_closed: bool,
    pub balance: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub version: u64,
}

#[derive(Identifiable)]
#[table_name = "bank_accounts"]
#[primary_key(id)]
struct UpdateExpense<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub is_closed: bool,
    pub balance: i32,
    pub updated_at: DateTime<Local>,
    pub version: u64,
}

impl Queryable<bank_accounts::SqlType, diesel::mysql::Mysql> for BankAccountRecord {
    type Row = (String, String, bool, i32, DateTime<Local>, DateTime<Local>, u64);

    fn build(row: Self::Row) -> Self {
        BankAccountRecord {
            id: row.0,
            name: row.1,
            is_closed: row.2,
            balance: row.3,
            created_at: row.4,
            updated_at: row.5,
            version: row.6,
        }
    }
}
