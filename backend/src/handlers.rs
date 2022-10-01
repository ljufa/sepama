use actix_web::{web, HttpResponse, Responder};

use crate::auth;
use crate::AppState;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, Set, Unchanged};

use log::{debug, error};
use serde::Serialize;

use entity::{
    sea_orm::{prelude::Date, ColumnTrait, EntityTrait, QueryFilter},
    user_profile::{self, Entity as UserProfile, Model},
};
use user_profile::Column::AuthId;

#[derive(Serialize)]
pub struct DtoBankAccount {
    pub institution: String,
    pub iban: String,
    pub bic: Option<String>,
}

pub mod profile {

    use api_models::validator::Validate;

    use super::*;

    pub async fn profile_exists(state: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
        return match get_profile_by_auth(&auth, &state).await {
            Ok((Some(_), _)) => HttpResponse::Ok(),
            Ok((None, _)) => HttpResponse::NotFound(),
            Err(_) => HttpResponse::InternalServerError(),
        };
    }

    pub async fn set_user_profile(
        state: web::Data<AppState>,
        dto: web::Json<api_models::models::UserProfile>,
        auth: BearerAuth,
    ) -> impl Responder {
        if dto.validate().is_err() {
            return HttpResponse::BadRequest();
        }
        let (id, auth_id) = match get_profile_by_auth(&auth, &state).await {
            Ok((Some(e), _)) => (Unchanged(e.id), Unchanged(e.auth_id)),
            Ok((None, a)) => (NotSet, Set(a)),
            Err(err) => {
                log::error!("Db Error: {}", err);
                return HttpResponse::InternalServerError();
            }
        };

        let new_profile = user_profile::ActiveModel {
            id,
            auth_id,
            address: Set(serde_json::to_value(&dto.address).ok()),
            date_created: NotSet,
            date_of_birth: Set(dto
                .date_of_birth
                .clone()
                .map(|d| Date::parse_from_str(d.as_str(), "%Y/%m/%d").ok().unwrap())),
            firstname: Set(dto.first_name.clone()),
            lastname: Set(dto.last_name.clone()),
            preferred_language: Set(dto.preferred_language.clone()),
            status: Set(user_profile::ProfileStatus::ProfileIncomplete),
        };
        let np = new_profile.save(&state.connection).await;
        debug!("Saved {:?}", np);
        return HttpResponse::Ok();
    }

    pub async fn get_user_profile(auth: BearerAuth, state: web::Data<AppState>) -> impl Responder {
        return match get_profile_by_auth(&auth, &state).await {
            Ok((Some(user), _)) => {
                let result = api_models::models::UserProfile {
                    address: user
                        .address
                        .clone()
                        .map(|ajson| serde_json::from_value(ajson).unwrap()),
                    date_of_birth: user.date_of_birth.map(|db| db.to_string()),
                    first_name: user.firstname.clone(),
                    last_name: user.lastname.clone(),
                    preferred_language: user.preferred_language.clone(),
                };
                return HttpResponse::Ok().json(web::Json(result).0);
            }
            Ok((None, _)) => HttpResponse::NotFound().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        };
    }

    pub async fn get_profile_by_auth(
        auth: &BearerAuth,
        state: &web::Data<AppState>,
    ) -> Result<(Option<Model>, String), entity::sea_orm::DbErr> {
        let td = auth::get_token_data(auth.token())
            .await
            .expect("Token data is not valid");
        let auth_id = td
            .claims
            .get("sub")
            .expect("Invalid token format!")
            .to_string();
        let existing = UserProfile::find()
            .filter(AuthId.eq(auth_id.clone()))
            .one(&state.connection)
            .await?;
        return Ok((existing, auth_id.clone()));
    }
}

pub mod mandate {
    use std::str::FromStr;

    use super::*;

    use api_models::{
        models::{Mandate as MandateDto, Status},
        validator::Validate,
    };
    use entity::{
        mandate::ActiveModel as MandateActiveModel,
        mandate::{Column, Entity as MandateEntity, MandateStatus},
        sea_orm::{ModelTrait, QueryOrder},
    };
    use serde_json::json;

    pub async fn get_mandates(_auth: BearerAuth, _state: web::Data<AppState>) -> impl Responder {
        let up = match profile::get_profile_by_auth(&_auth, &_state).await {
            Ok((Some(up), _)) => up,
            Ok((None, _)) => return HttpResponse::Forbidden().finish(),
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
        let result = up
            .find_related(MandateEntity)
            .order_by_asc(Column::DateCreated)
            .all(&_state.connection)
            .await;
        return match result {
            Ok(res) => {
                let mandates: Vec<MandateDto> = res
                    .iter()
                    .map(|m| {
                        MandateDto {
                            api_id: m.api_id,
                            status: Status::from_str(m.status.clone().into())
                                .expect("Unknown mandate state"),
                            unique_reference: m.unique_reference.clone(),
                            display_name: m.display_name.clone(),
                            date_created: Some(m.date_created.to_string()), // todo ?
                            creditor: serde_json::from_value(m.creditor.clone())
                                .unwrap_or_default(),
                            bank_account: serde_json::from_value(m.bank_account.clone())
                                .unwrap_or_default(),
                            tags: m
                                .tags
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|st| st.as_str().unwrap().to_owned())
                                .collect(),
                        }
                    })
                    .collect();
                HttpResponse::Ok().json(mandates)
            }
            Err(e) => {
                error!("Error returning mandates for {} {:?}", up.id, e);
                HttpResponse::InternalServerError().finish()
            }
        };
    }

    pub async fn save_mandate(
        auth: BearerAuth,
        state: web::Data<AppState>,
        dto: web::Json<MandateDto>,
    ) -> impl Responder {
        if let Err(_err) = dto.validate() {
            // todo errors to body message
            return HttpResponse::BadRequest();
        }
        let user_profile = match profile::get_profile_by_auth(&auth, &state).await {
            Ok((Some(up), _)) => up,
            Ok((None, _)) => return HttpResponse::Forbidden(),
            Err(_) => return HttpResponse::InternalServerError(),
        };
        let matched_mandate = user_profile
            .find_related(MandateEntity)
            .filter(entity::mandate::Column::ApiId.eq(dto.api_id))
            .one(&state.connection)
            .await;
        let (id, api_id, user_profile_id) = match matched_mandate {
            Ok(Some(m)) => (
                Unchanged(m.id),
                Unchanged(m.api_id),
                Unchanged(m.user_profile_id),
            ),
            Ok(None) => (NotSet, Set(dto.api_id), Set(user_profile.id)),
            Err(e) => {
                error!("Error fetching mandate {}, {:?}", dto.api_id, e);
                return HttpResponse::InternalServerError();
            }
        };

        let active_model = mandate::MandateActiveModel {
            id,
            api_id,
            user_profile_id,
            tags: Set(json!(dto.tags)),
            status: Set(
                MandateStatus::from_str(dto.status.clone().into()).expect("Unknown mandate status")
            ),
            unique_reference: Set(dto.unique_reference.clone()),
            display_name: Set(dto.display_name.clone()),
            date_created: NotSet,
            creditor: Set(json!(dto.creditor)),
            bank_account: Set(json!(dto.bank_account)),
        };

        let result = active_model.save(&state.connection).await;
        return match result {
            Ok(_) => HttpResponse::Ok(),
            Err(e) => {
                error!("Error persisting dto {:?} {}", dto, e);
                HttpResponse::InternalServerError()
            }
        };
    }
}
