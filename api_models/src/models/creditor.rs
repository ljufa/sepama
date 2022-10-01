use validator::Validate;

use super::Address;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct Creditor {

    #[validate(length(min = 2))]
    pub name: String,
    
    #[validate(length(min = 2))]
    pub sepa_identifier: Option<String>,
    
    #[validate]
    pub address: Address,
}
