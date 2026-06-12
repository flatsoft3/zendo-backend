use serde::{Deserialize, Serialize};
use strum::Display;


#[derive(Deserialize, Serialize,  Debug, Clone, Copy, Display, PartialEq, Eq, strum::AsRefStr)]
pub enum PaymentGateway {
    Korapay
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Display, PartialEq, Eq, strum::AsRefStr)]
pub enum Currency {
    #[strum(serialize = "NGN")]
    NGN
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::AsRefStr)]
//#[strum(serialize_all = "lowercase")] // 👈 This converts all variants to lowercase automatically!
pub enum PaymentStatus {
    #[strum(serialize = "Success")]
    Success,
    #[strum(serialize = "Pending")]
    Pending,
    #[strum(serialize = "Failed")]
    Failed,
    #[strum(serialize = "Reversed")]
    Reversed
}