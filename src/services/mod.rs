use strum_macros::{EnumString, IntoStaticStr};

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ServiceName {
    Unknown,

    Budget,
    Event,
    Expense,
    Strategy,
    Todo,
}
