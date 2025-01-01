use crate::domain::employee::request::CreateNewEmployeeByUserUuidRequest;
use crate::core::response::{ClientResponseError, MessageResponse, ServiceStatusResponse};
use crate::domain::user::request::{RegisterRequest, UpdateProfileRequest};
use crate::domain::user::response::{
    GetUserListResponse, GetUserResponse, ProfileResponse, RegisterResponse,
};
use crate::util::filter_and_pagination::Direction;
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        version = "1.0.0",
        title = "UPTOP ERP API",
    ),
    paths(
        crate::controller::server::handler::health_check,
        crate::controller::server::handler::server_state,
        crate::controller::user::handler::register_by_email,
        crate::controller::user::handler::get_profile,
        crate::controller::user::handler::get_list,
        crate::controller::user::handler::update_profile,
        crate::controller::employee::handler::create_new_employee_by_user_exist,
    ),
    components(
        schemas(
            RegisterRequest,
            RegisterResponse,
            ClientResponseError,
            MessageResponse,
            ProfileResponse,
            UpdateProfileRequest,
            ServiceStatusResponse,
            GetUserResponse,
            GetUserListResponse,
            Direction,
            CreateNewEmployeeByUserUuidRequest
        )
    ),
    tags(
        (name = "server_service", description = "server endpoints."),
        (name = "user_service", description = "user endpoints."),
        (name = "employee_service", description = "employee endpoints."),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components
            .add_security_scheme("jwt", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
    }
}
