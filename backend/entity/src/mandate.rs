
use sea_orm::{entity::prelude::*, strum::{EnumString, IntoStaticStr}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "mandate")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(unique)]
    pub api_id: Uuid,

    pub user_profile_id: i32,

    pub tags: Json,

    pub status: MandateStatus,

    pub unique_reference: Option<String>,

    pub display_name: String,

    pub date_created: DateTime,

    pub creditor: Json,

    pub bank_account: Json,
}

#[derive(EnumIter, IntoStaticStr, EnumString, DeriveActiveEnum, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum MandateStatus {
    #[sea_orm(string_value = "ACTIVE")]
    ACTIVE,

    #[sea_orm(string_value = "DELETED")]
    DELETED,

    #[sea_orm(string_value = "CANCELED")]
    CANCELED,
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserProfile,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserProfile => Entity::belongs_to(super::user_profile::Entity)
                .from(Column::UserProfileId)
                .to(super::user_profile::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
