use strum_macros::{EnumString, IntoStaticStr};

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumString)]
pub enum ServiceName {
    Unknown,

    Budget,
    Event,
    Expense,
    Strategy,
    Todo,
}
