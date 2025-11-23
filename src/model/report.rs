use crate::model::account::Account;
use std::fmt;

pub struct Report {
    accounts: Vec<Account>,
}

impl Report {
    pub fn new(accounts: Vec<Account>) -> Self {
        Self { accounts }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "client,available,held,total,locked")?;

        for account in &self.accounts {
            writeln!(
                f,
                "{},{},{},{},{}",
                account.client, account.available, account.held, account.total, account.locked
            )?
        }

        Ok(())
    }
}
