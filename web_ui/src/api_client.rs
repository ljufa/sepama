use api_models::models::{UserProfile, Mandate};
use seed::{prelude::*, *};

use crate::{User, AuthError};

const API_URL_MANDATES: &str = "/api/mandates";
const API_URL_PROFILE: &str = "/api/profile";

// ------ ------
//     API calls
// ------ ------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn init_auth(
        domain: String,
        client_id: String,
        audience: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn redirect_to_sign_up() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn redirect_to_log_in() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn logout() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn getTokenSilently() -> Result<JsValue, JsValue>;

}

pub async fn get_token() -> fetch::Result<String> {
    match getTokenSilently().await {
        Ok(token) => Ok(token.as_string().unwrap()),
        Err(err) => Err(fetch::FetchError::NetworkError(err)),
    }
}

pub async fn get_auth_user() -> Result<User, AuthError> {
    let domain = String::from("dev-i-l2f2pc.eu.auth0.com");
    let client_id = String::from("P1wEshlqr9kmr8C8WcJJ6aMmZ3ADFkrn");
    let audience = String::from("http://mysepa-backend/api");   
    let js_user = init_auth(domain, client_id, audience).await;
    log!("Auth JSValue {}", js_user);
    if js_user.is_err() {
        return Err(AuthError::ConfigurationError);
    }
    let js_user = js_user.unwrap();
    if not(js_user.is_undefined()) {
        match serde_wasm_bindgen::from_value(js_user) {
            Ok(user) => {
                return Ok(user);
            }
            Err(_error) => return Err(AuthError::UnparsableJson),
        }
    }
    Err(AuthError::NotAuthenticated)
}

pub async fn is_profile_created() -> fetch::Result<Status> {
    Ok(Request::new(API_URL_PROFILE)
        .method(Method::Head)
        .header(Header::bearer(get_token().await?))
        .header(Header::custom("Accept", "application/json"))
        .header(Header::content_type("application/json"))
        .fetch()
        .await?
        .check_status()?
        .status())
}


pub async fn save_profile(user_profile: UserProfile) -> fetch::Result<Status> {
    Ok(Request::new(API_URL_PROFILE)
        .method(Method::Post)
        .header(Header::custom("Accept", "application/json"))
        .header(Header::content_type("application/json"))
        .header(Header::bearer(get_token().await?))
        .json(&user_profile)?
        .fetch()
        .await?
        .check_status()?
        .status())
}
pub async fn get_user_profile() -> fetch::Result<UserProfile> {
    Request::new(API_URL_PROFILE)
        .method(Method::Get)
        .header(Header::bearer(get_token().await?))
        .header(Header::custom("Accept", "application/json"))
        .header(Header::content_type("application/json"))
        .fetch()
        .await?
        .check_status()?
        .json::<UserProfile>()
        .await
}


pub async fn request_mandates() -> fetch::Result<Vec<Mandate>> {
    match getTokenSilently().await {
        Ok(token) => {
            Request::new(API_URL_MANDATES)
                .method(Method::Get)
                .header(Header::custom("Accept", "application/json"))
                .header(Header::content_type("application/json"))
                .header(Header::bearer(token.as_string().unwrap()))
                .fetch()
                .await?
                .check_status()?
                .json::<Vec<Mandate>>()
                .await
        }
        Err(err) => Err(fetch::FetchError::NetworkError(err)),
    }
}

pub async fn save_selected_mandate(mandate: Mandate) -> fetch::Result<Response> {
    match getTokenSilently().await {
        Ok(token) => Request::new(API_URL_MANDATES)
            .method(Method::Post)
            .header(Header::custom("Accept", "application/json"))
            .header(Header::content_type("application/json"))
            .header(Header::bearer(token.as_string().unwrap()))
            .json(&mandate)?
            .fetch()
            .await?
            .check_status(),
        Err(err) => Err(fetch::FetchError::NetworkError(err)),
    }
}
