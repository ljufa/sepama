use validator::Validate;

use super::BankAccount;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct Mandate {

    pub api_id: uuid::Uuid,
    
    pub tags: Vec<String>,

    pub status: crate::models::Status,
    
    #[validate(length(min = 2))]
    pub unique_reference: Option<String>,
    
    #[validate(length(min = 2))]
    pub display_name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,

    #[validate]
    pub creditor: crate::models::Creditor,

    #[validate]
    pub bank_account: BankAccount,

}
