use chrono::{DateTime, Utc, Local};

use elastic::client::SyncClient;
use elastic::prelude::*;

use serde::{Serialize, Deserialize};

use rust_cqrses_bankaccount::dao::{BankAccountRM, BankAccountDao};

pub struct ElasticBankAccountDao {
    client: SyncClient,
}

impl ElasticBankAccountDao {
    pub fn new(client: SyncClient) -> Self {
        Self {
            client,
        }
    }
}

impl BankAccountDao for ElasticBankAccountDao {
    fn find(&self, bank_account_id: String) -> Option<BankAccountRM> {
        let response = self.client.document::<BankAccountRecord>().get(bank_account_id).send().unwrap();
        match response.into_document() {
            Some(doc) => Some(BankAccountRM {
                bank_account_id: doc.bank_account_id.clone(),
                name: doc.name.clone(),
                is_closed: doc.is_closed,
                balance: doc.balance,
                created_at: DateTime::parse_from_rfc3339(&format!("{}", doc.created_at)).unwrap().with_timezone(&Local),
                updated_at: DateTime::parse_from_rfc3339(&format!("{}", doc.updated_at)).unwrap().with_timezone(&Local),
                version: doc.version.parse().unwrap(),
            }),
            None => None,
        }
    }

    fn insert(&self, model: BankAccountRM) {
        let doc = BankAccountRecord {
            bank_account_id: model.bank_account_id.clone(),
            name: model.name.clone(),
            is_closed: model.is_closed,
            balance: model.balance,
            created_at: Date::new(model.created_at.with_timezone(&Utc)),
            updated_at: Date::new(model.updated_at.with_timezone(&Utc)),
            version: model.version.to_string(),
        };

        let response = self.client.document().index(doc).send();
        response.unwrap();
    }

    fn update(&self, model: BankAccountRM) {
        let new_doc = BankAccountRecord {
            bank_account_id: model.bank_account_id.clone(),
            name: model.name.clone(),
            is_closed: model.is_closed,
            balance: model.balance,
            created_at: Date::new(model.created_at.with_timezone(&Utc)),
            updated_at: Date::new(model.updated_at.with_timezone(&Utc)),
            version: model.version.to_string(),
        };

        let response = self.client.document::<BankAccountRecord>()
            .update(model.bank_account_id.clone())
            .doc(new_doc)
            .send();

        response.unwrap();
    }
}

#[derive(Serialize, Deserialize, ElasticType)]
struct BankAccountRecord {
    #[elastic(id)]
    pub bank_account_id: String,
    pub name: String,
    pub is_closed: bool,
    pub balance: i32,
    pub created_at: Date<DefaultDateMapping<ChronoFormat>>,
    pub updated_at: Date<DefaultDateMapping<ChronoFormat>>,
    pub version: String,
}
