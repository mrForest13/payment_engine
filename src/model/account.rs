use crate::model::client::ClientId;
use rust_decimal::Decimal;

pub struct Account {
    pub client: ClientId,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}
