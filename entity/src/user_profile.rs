use sea_orm::{prelude::*, DeriveEntityModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub auth_id: String,
    pub date_created: DateTime,
    pub firstname: String,
    pub lastname: String,
    pub address: Option<Json>,
    pub preferred_language: Option<String>,
    pub date_of_birth: Option<Date>,
    pub status: ProfileStatus,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ProfileStatus {
    #[sea_orm(string_value = "PROFILE_INCOMPLETE")]
    ProfileIncomplete,
    #[sea_orm(string_value = "PROFILE_COMPLETE")]
    ProfileComplete,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Mandates,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Mandates => Entity::has_many(super::mandate::Entity).into(),
        }
    }
}

impl Related<super::mandate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mandates.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
