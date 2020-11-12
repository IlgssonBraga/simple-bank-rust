// use crate::client;

pub struct Account {
    agency: u32,
    number: u32,
    balance: u32,
}

impl Account {
    pub fn get_agency(&self) -> &u32 {
        &self.agency
    }

    pub fn get_number(&self) -> &u32 {
        &self.number
    }

    pub fn get_balance(&self) -> &u32 {
        &self.balance
    }

    pub fn create_account(agency: u32, number: u32, balance: u32) -> Account {
        Account {
            agency,
            number,
            balance,
        }
    }
}