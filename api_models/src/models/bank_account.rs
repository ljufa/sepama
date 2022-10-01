use lazy_static::lazy_static;
use regex::Regex;
use validator::Validate;

lazy_static! {
     static ref RE_IBAN: Regex = Regex::new(r"^[A-Z]{2}[0-9]{2}(?:[ ]?[0-9]{4}){4}(?:[ ]?[0-9]{1,2})?$").unwrap();
     static ref RE_BIC: Regex = Regex::new(r"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
}


#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Default, Serialize, Deserialize, Validate)]
pub struct BankAccount {

    #[validate(length(min = 2))]
    pub institution: String,
    
    #[validate(regex(path = "RE_IBAN"))]
    pub iban: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex(path = "RE_BIC"))]
    pub bic: Option<String>,

}

