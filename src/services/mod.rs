use crate::operations::Type;
use clap::ArgMatches;
use strum_macros::IntoStaticStr;

pub mod budget;
pub mod expense;
pub mod todo;

#[derive(PartialEq, Eq, Hash, IntoStaticStr)]
pub enum Name {
    Budget,
    Event,
    Expense,
    Strategy,
    Todo,
    Unknown,
}

pub trait Service {
    fn name(&self) -> Name {
        Name::Unknown
    }
    fn apply(&self, _op_type: Type, _arg_matches: &ArgMatches) -> bool {
        false
    }
    fn get_supported_op_types(&self) -> Vec<Type> {
        vec![]
    }
}
