use seed::{prelude::*, *};

use crate::{page::view_validation_icon, api_client};
use api_models::{
    models::{Address, UserProfile},
    validator::Validate,
};

#[derive(Default)]
pub struct Model {
    user_profile: UserProfile,
    fetching_remote_data: bool,
    saving_remote_data: bool,
    saved_success: bool,
}
#[derive(Debug)]
pub enum Msg {
    FirstNameChanged(String),
    LastNameChanged(String),
    StreetChanged(String),
    ZipChanged(String),
    PlaceChanged(String),
    HouseNoChanged(String),
    SaveProfile,
    ProfileSaved(fetch::Result<Status>),
    ProfileFetched(fetch::Result<UserProfile>),
    RemoveNotification,
}

pub fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::ProfileFetched(api_client::get_user_profile().await) });
    Model {
        user_profile: UserProfile::default(),
        fetching_remote_data: true,
        saving_remote_data: false,
        saved_success: false,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FirstNameChanged(value) => model.user_profile.first_name = value,
        Msg::LastNameChanged(value) => model.user_profile.last_name = value,
        Msg::StreetChanged(value) => {
            model
                .user_profile
                .address
                .get_or_insert(Address::default())
                .street = value;
        }
        Msg::ZipChanged(value) => {
            model
                .user_profile
                .address
                .get_or_insert(Address::default())
                .zip = value
        }
        Msg::PlaceChanged(value) => {
            model
                .user_profile
                .address
                .get_or_insert(Address::default())
                .place = value
        }
        Msg::HouseNoChanged(value) => {
            model
                .user_profile
                .address
                .get_or_insert(Address::default())
                .house_number = value
        }
        Msg::SaveProfile => {
            model.saving_remote_data = true;
            let up = model.user_profile.clone();
            orders.perform_cmd(async { Msg::ProfileSaved(api_client::save_profile(up).await) });
        }
        Msg::ProfileFetched(user_profile) => {
            log!("Fetch me {}", user_profile);
            model.fetching_remote_data = false;
            match user_profile {
                Ok(up) => model.user_profile = up,
                Err(e) => log!("error in profile get {}", e),
            }
        }
        Msg::ProfileSaved(result) => {
            model.saving_remote_data = false;
            model.saved_success = true;
            orders.stream(streams::interval(3000, || Msg::RemoveNotification));
            log!("profile saved {}", result)
        }
        Msg::RemoveNotification => {
            model.saved_success = false;
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    if model.fetching_remote_data {
        return progress![C!["progress", "is-link", "mt-6"]];
    }
    let profile = &model.user_profile;
    let address = if let Some(adr) = &profile.address {
        adr.clone()
    } else {
        Address::default()
    };
    div![
        C!["column"],
        div![
            C!["box"],
            nav![
                C!["level"],
                div![
                    C!["level-left"],
                    // First Name
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            label![C!["label"], "First name"],
                            div![
                                C!["control", "has-icons-left has-icons-right"],
                                input![
                                    C!["input", "is-success"],
                                    attrs! {
                                        At::Value => profile.first_name
                                    },
                                    input_ev(Ev::Input, move |value| {
                                        Msg::FirstNameChanged(value)
                                    }),
                                ],
                                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-user"]]],
                                view_validation_icon(profile, "first_name")
                            ]
                        ],
                    ],
                    // Last name
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            label![C!["label"], "Last name"],
                            div![
                                C!["control", "has-icons-left", "has-icons-right"],
                                input![
                                    C!["input", "is-success"],
                                    attrs! {
                                        At::Value => profile.last_name
                                    },
                                    input_ev(Ev::Input, move |value| {
                                        Msg::LastNameChanged(value)
                                    }),
                                ],
                                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-user"]]],
                                view_validation_icon(profile, "last_name")
                            ]
                        ]
                    ]
                ]
            ],
            // address
            nav![
                C!["level"],
                div![
                    C!["level-left"],
                    // Street
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            label![C!["label"], "Street"],
                            div![
                                C!["control", "has-icons-left has-icons-right"],
                                input![
                                    C!["input", "is-success"],
                                    attrs! {
                                        At::Value => address.street
                                    },
                                    input_ev(Ev::Input, move |value| { Msg::StreetChanged(value) }),
                                ],
                                span![
                                    C!["icon", "is-small", "is-left"],
                                    i![C!["fas", "fa-map-marker-alt"]]
                                ],
                                view_validation_icon(&address, "street")
                            ]
                        ],
                    ],
                    // Hause number
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            label![C!["label"], "House number"],
                            div![
                                C!["control", "has-icons-left has-icons-right"],
                                input![
                                    C!["input"],
                                    attrs! {
                                        At::Value => address.house_number
                                    },
                                    input_ev(Ev::Input, move |value| {
                                        Msg::HouseNoChanged(value)
                                    }),
                                ],
                                span![
                                    C!["icon", "is-small", "is-left"],
                                    i![C!["fas", "fa-map-marker-alt"]]
                                ],
                                view_validation_icon(&address, "house_number")
                            ]
                        ]
                    ]
                ]
            ],
            nav![
                C!["level"],
                div![
                    C!["level-left"],
                    // Zip code
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            label![C!["label"], "Zip"],
                            div![
                                C!["control", "has-icons-left has-icons-right"],
                                input![
                                    C!["input", "is-success"],
                                    attrs! {
                                        At::Value => address.zip
                                    },
                                    input_ev(Ev::Input, move |value| { Msg::ZipChanged(value) }),
                                    // ev(Ev::Change, move |_| Msg::SaveCreditor),
                                ],
                                span![
                                    C!["icon", "is-small", "is-left"],
                                    i![C!["fas", "fa-map-marker-alt"]]
                                ],
                                view_validation_icon(&address, "zip")
                            ]
                        ],
                    ],
                    // Place
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            label![C!["label"], "Place"],
                            div![
                                C!["control", "has-icons-left has-icons-right"],
                                input![
                                    C!["input"],
                                    attrs! {
                                        At::Value => address.place
                                    },
                                    input_ev(Ev::Input, move |value| { Msg::PlaceChanged(value) }),
                                    // ev(Ev::Change, move |_| Msg::SaveCreditor),
                                ],
                                span![
                                    C!["icon", "is-small", "is-left"],
                                    i![C!["fas", "fa-map-marker-alt"]]
                                ],
                                view_validation_icon(&address, "place")
                            ]
                        ]
                    ]
                ]
            ],
        ],
        // buttons
        div![
            C!["field", "is-grouped", "section"],
            div![
                C!["control"],
                button![
                    IF!(model.user_profile.validate().is_err() => attrs!{ At::Disabled => ""}),
                    C![
                        "button",
                        "is-success",
                        IF!(model.saving_remote_data => "is-loading")
                    ],
                    "Save",
                    ev(Ev::Click, |_| Msg::SaveProfile),
                ]
            ],
            div![C!["control"], button![C!["button", "is-danger"], "Delete"]],
        ],
        IF!(model.saved_success => div![C!["notification", "is-success", "is-light"],"Profile saved successfully."]),
    ]
}

