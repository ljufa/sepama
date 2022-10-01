use validator::Validate;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct Address {
    #[validate(length(min = 2))]
    pub street: String,

    #[validate(length(min = 2))]
    pub house_number: String,

    #[validate(length(min = 2))]
    pub zip: String,

    #[validate(length(min = 2))]
    pub place: String,
}
