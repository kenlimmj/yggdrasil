use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum OperationType {
    Unknown,

    Add,
    Link,
    List,
    Remove,
}
