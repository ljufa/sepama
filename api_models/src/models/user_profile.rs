use lazy_static::lazy_static;
use regex::Regex;
use validator::Validate;



lazy_static! {
    static ref RE_DATE: Regex = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap();
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct UserProfile {
    #[validate(length(min = 2))]
    pub first_name: String,

    #[validate(length(min = 2))]
    pub last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub address: Option<crate::models::Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex(path = "RE_DATE"))]
    pub date_of_birth: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(equal = 2))]
    pub preferred_language: Option<String>,
}

impl UserProfile {
    pub fn new(first_name: String, last_name: String) -> UserProfile {
        UserProfile {
            first_name,
            last_name,
            address: None,
            date_of_birth: None,
            preferred_language: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::Address;
    use validator::Validate;

    use super::UserProfile;

    #[test]
    fn test_validate_user_profile_date_of_birth() {
        let mut up = UserProfile::new("Dragan".to_string(), "Ljub".to_string());
        assert_eq!(Ok(()), up.validate());

        up.date_of_birth = Some("20-01-1902".to_string());
        assert_eq!(Ok(()), up.validate());

        up.date_of_birth = Some("1924-01-19".to_string());
        let res = up.validate();
        assert_eq!(
            true,
            res.unwrap_err().errors().contains_key("date_of_birth")
        );
    }

    #[test]
    fn test_validate_user_profile_first_name() {
        let mut up = UserProfile::new("Dragan".to_string(), "Ljub".to_string());
        assert_eq!(Ok(()), up.validate());

        up.first_name = "D".to_string();
        assert_eq!(
            true,
            up.validate()
                .unwrap_err()
                .errors()
                .contains_key("first_name")
        );
    }

    #[test]
    fn test_validate_user_profile_last_name() {
        let mut up = UserProfile::new("Dragan".to_string(), "Ljub".to_string());
        assert_eq!(Ok(()), up.validate());

        up.last_name = "D".to_string();
        assert_eq!(
            true,
            up.validate()
                .unwrap_err()
                .errors()
                .contains_key("last_name")
        );
    }

    #[test]
    fn test_validate_address_when_present() {
        let mut up = UserProfile::new("Dragan".to_string(), "Ljub".to_string());
        up.address = Some(Address::default());
        assert_eq!(
            true,
            up.validate().unwrap_err().errors().contains_key("address")
        );
    }
    
}
