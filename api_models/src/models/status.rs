use strum_macros::{IntoStaticStr, EnumString};

#[derive(Clone, IntoStaticStr, EnumString, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    ACTIVE,
    DELETED,
    CANCELED,
    NEW
}


impl Default for Status {
    fn default() -> Status {
        Self::NEW
    }
}
