use crate::core::response::{
    ClientResponseError, EntityResponse, MessageResponse, ServiceStatusResponse,
};
use crate::domain::authenticate::request::{LoginByEmailRequest, RefreshTokenRequest};
use crate::domain::authenticate::response::{LoginResponse, TokenResponse};
use crate::domain::employee::request::{
    CreateNewEmployeeByUserUuidRequest, CreateNewEmployeeRequest, DeleteEmployeeRequest,
    UpdateEmployeeRequest,
};
use crate::domain::user::request::{AdminCreateAccountRequest, UpdateProfileRequest};
use crate::domain::user::response::PublicProfileResponse;
use crate::util::filter_and_pagination::{Direction, PageQueryParam};
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
        // server api
        crate::controller::server::health_check,
        crate::controller::server::server_state,
        // auth api
        crate::controller::auth::controller_login_by_email,
        crate::controller::auth::controller_refresh_token,
        // user api
        crate::controller::admin::user::controller_admin_create_account,
        crate::controller::admin::user::controller_admin_get_list,
        crate::controller::user::controller_get_profile,
        crate::controller::user::controller_update_profile,
        crate::controller::user::controller_logout,

        // employee api
        crate::controller::employee::create_new_employee_by_user_exist,
        crate::controller::employee::create_new_employee,
        crate::controller::employee::update_employee,
        crate::controller::employee::delete_employee,
    ),
    components(
        schemas(
            // request
            LoginByEmailRequest,
            RefreshTokenRequest,
            AdminCreateAccountRequest,
            UpdateProfileRequest,
            CreateNewEmployeeRequest,
            CreateNewEmployeeByUserUuidRequest,
            UpdateEmployeeRequest,
            DeleteEmployeeRequest,
            Direction,
            PageQueryParam,

            // response
            LoginResponse,
            TokenResponse,
            MessageResponse,
            EntityResponse<Vec<PublicProfileResponse>>,
            PublicProfileResponse,
            ServiceStatusResponse,
            ClientResponseError,
        )
    ),
    tags(
        (name = "server_service", description = "server endpoints."),
        (name = "auth_service", description = "authenticate endpoints."),
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
