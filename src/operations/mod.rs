use strum_macros::{EnumString, IntoStaticStr};

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumString)]
pub enum OperationType {
    Unknown,

    Add,
    Link,
    List,
    Remove,
}
