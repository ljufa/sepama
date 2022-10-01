#![allow(clippy::wildcard_imports)]

use page::{sepa_management, user_profile};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

mod page;
mod api_client;

const SEPA_MANAGEMENT: &str = "manage";
const USER_PROFILE: &str = "user-profile";

// ------ ------~
//     Model
// ------ ------

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    nickname: String,
    name: String,
    picture: String,
    updated_at: String,
    sub: String,
    email: String,
    email_verified: bool,
}

pub struct Model {
    user: Option<User>,
    is_profile_created: bool,
    base_url: Url,
    page: Page,
    menu_visible: bool,
    remote_call_in_progress: bool,
}

// ------ Page ------
enum Page {
    Home,
    SepaManagement(sepa_management::Model),
    UserProfile(user_profile::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [SEPA_MANAGEMENT] => Self::SepaManagement(page::sepa_management::init(
                url,
                &mut orders.proxy(Msg::SepaManagement),
            )),
            [USER_PROFILE] => Self::UserProfile(page::user_profile::init(
                url,
                &mut orders.proxy(Msg::UserProfile),
            )),
            _ => Self::NotFound,
        }
    }
}

#[derive(Debug, Display)]
enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleMenu,
    HideMenu,
    AuthInitFinished(Result<User, AuthError>),

    SignUp,
    LogIn,
    LogOut,
    IsProfileExistsFetched(fetch::Result<Status>),
    RedirectingToSignUp(Result<(), JsValue>),
    RedirectingToLogIn(Result<(), JsValue>),
    // ------ pages ------
    SepaManagement(sepa_management::Msg),
    UserProfile(user_profile::Msg),
}

#[derive(Debug)]
pub enum AuthError {
    NotAuthenticated,
    ConfigurationError,
    UnparsableJson,
}

// ------ ------
//     Init
// ------ ------
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu))
        .perform_cmd(async { Msg::AuthInitFinished(api_client::get_auth_user().await) });
    Model {
        user: None,
        base_url: url.to_base_url(),
        page: Page::Home,
        menu_visible: false,
        remote_call_in_progress: true,
        is_profile_created: false,
    }
}
// ------ ------
//    Update
// ------ ------
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log!("lib.rs - update {}", msg.to_string());
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::ToggleMenu => model.menu_visible = not(model.menu_visible),
        Msg::HideMenu => {
            if model.menu_visible {
                model.menu_visible = false;
            } else {
                orders.skip();
            }
        }

        Msg::AuthInitFinished(Ok(user)) => {
            model.user = Some(user);
            model.remote_call_in_progress = true;
            orders.perform_cmd(async { Msg::IsProfileExistsFetched(api_client::is_profile_created().await) });
            let search = model.base_url.search_mut();
            if search.remove("code").is_some() && search.remove("state").is_some() {
                model.base_url.go_and_replace();
            }
        }
        Msg::AuthInitFinished(Err(error)) => {
            log!("Auth initialization failed {}", error);
        }

        Msg::SignUp => {
            orders.perform_cmd(async { Msg::RedirectingToSignUp(api_client::redirect_to_sign_up().await) });
        }
        Msg::LogIn => {
            orders.perform_cmd(async { Msg::RedirectingToLogIn(api_client::redirect_to_log_in().await) });
        }
        Msg::RedirectingToSignUp(result) => {
            if let Err(error) = result {
                error!("Redirect to sign up failed!", error);
            }
        }
        Msg::RedirectingToLogIn(result) => {
            if let Err(error) = result {
                error!("Redirect to log in failed!", error);
            }
        }
        Msg::LogOut => {
            if let Err(error) = api_client::logout() {
                error!("Cannot log out!", error);
            } else {
                model.user = None;
            }
        }
        Msg::IsProfileExistsFetched(e) => {
            model.remote_call_in_progress = false;
            match e {
                Ok(st) => model.is_profile_created = st.is_ok(),
                Err(_) => model.is_profile_created = false,
            }
        }

        Msg::SepaManagement(msg) => {
            if let Page::SepaManagement(model) = &mut model.page {
                page::sepa_management::update(msg, model, &mut orders.proxy(Msg::SepaManagement))
            }
        }
        Msg::UserProfile(msg) => {
            if let Page::UserProfile(model) = &mut model.page {
                page::user_profile::update(msg, model, &mut orders.proxy(Msg::UserProfile))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_navbar(
            model.menu_visible,
            &model.base_url,
            model.user.as_ref(),
            &model.page,
        ),
        view_content(model),
        // view_footer(),
    ]
}

// ----- view_content ------
fn view_content(model: &Model) -> Node<Msg> {
    if model.user.is_none() {
        div![page::anonimous::view()]
    } else if model.remote_call_in_progress {
        progress![C!["progress", "is-link", "mt-6"]]
    } else {
        div![
            style! {St::Padding => "40pt"},
            C!["container"],
            match &model.page {
                Page::Home => page::home::view(model),
                Page::SepaManagement(mdl) => {
                    if model.is_profile_created {
                        page::sepa_management::view(mdl).map_msg(Msg::SepaManagement)
                    } else {
                        p!["Please create your profile!"]
                    }
                }
                Page::UserProfile(mdl) =>
                    page::user_profile::view(mdl).map_msg(Msg::UserProfile),
                Page::NotFound => page::not_found::view(),
            },
        ]
    }
}

// ----- view_navbar ------

fn view_navbar(menu_visible: bool, base_url: &Url, user: Option<&User>, page: &Page) -> Node<Msg> {
    nav![
        C!["navbar", "is-link"],
        attrs! {
            At::from("role") => "navigation",
            At::AriaLabel => "main navigation",
        },
        view_brand_and_hamburger(menu_visible, base_url),
        view_navbar_menu(menu_visible, base_url, user, page),
    ]
}

fn view_brand_and_hamburger(menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-brand"],
        // ------ Logo ------
        a![
            C!["navbar-item", "has-text-weight-bold", "is-size-3"],
            attrs! {At::Href => Urls::new(base_url).home()},
            "SEPAMAMAMAMA"
        ],
        // ------ Hamburger ------
        a![
            C!["navbar-burger", "burger", IF!(menu_visible => "is-active")],
            style! {
                St::MarginTop => "auto",
                St::MarginBottom => "auto",
            },
            attrs! {
                At::from("role") => "button",
                At::AriaLabel => "menu",
                At::AriaExpanded => menu_visible,
            },
            ev(Ev::Click, |event| {
                event.stop_propagation();
                Msg::ToggleMenu
            }),
            span![attrs! {At::AriaHidden => "true"}],
            span![attrs! {At::AriaHidden => "true"}],
            span![attrs! {At::AriaHidden => "true"}],
        ]
    ]
}

fn view_navbar_menu(
    menu_visible: bool,
    base_url: &Url,
    user: Option<&User>,
    page: &Page,
) -> Node<Msg> {
    div![
        C!["navbar-menu", IF!(menu_visible => "is-active")],
        view_navbar_menu_start(base_url, page, user),
        view_navbar_menu_end(base_url, user),
    ]
}

fn view_navbar_menu_start(base_url: &Url, page: &Page, user: Option<&User>) -> Node<Msg> {
    if user.is_none() {
        empty!()
    } else {
        div![
            C!["navbar-start"],
            a![
                C![
                    "navbar-item",
                    IF!(matches!(page, Page::SepaManagement(_)) => "is-active"),
                ],
                attrs! {At::Href => Urls::new(base_url).sepa_management()},
                "Manage sepa mandates",
            ],
            a![
                C![
                    "navbar-item",
                    IF!(matches!(page, Page::UserProfile(_)) => "is-active"),
                ],
                attrs! {At::Href => Urls::new(base_url).user_profile()},
                "My Profile",
            ],
        ]
    }
}

fn view_navbar_menu_end(base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["navbar-end"],
        div![
            C!["navbar-item"],
            div![if let Some(user) = user {
                img![attrs! {
                    At::Src => user.picture
                }]
            } else {
                empty![]
            }]
        ],
        div![
            C!["navbar-item"],
            div![
                C!["buttons"],
                if let Some(user) = user {
                    view_buttons_for_logged_in_user(base_url, user)
                } else {
                    view_buttons_for_anonymous_user()
                }
            ]
        ]
    ]
}

fn view_buttons_for_logged_in_user(_base_url: &Url, _user: &User) -> Vec<Node<Msg>> {
    vec![a![
        C!["button", "is-light"],
        "Log out",
        ev(Ev::Click, |_| Msg::LogOut),
    ]]
}

fn view_buttons_for_anonymous_user() -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", "is-primary"],
            strong!["Sign up"],
            ev(Ev::Click, |_| Msg::SignUp),
        ],
        a![
            C!["button", "is-light"],
            "Log in",
            ev(Ev::Click, |_| Msg::LogIn),
        ],
    ]
}
fn view_footer() -> Node<Msg> {
    div![raw!(include_str!("../web/templates/footer.html"))]
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn sepa_management(self) -> Url {
        self.base_url().add_path_part(SEPA_MANAGEMENT)
    }
    fn user_profile(self) -> Url {
        self.base_url().add_path_part(USER_PROFILE)
    }
}




// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
