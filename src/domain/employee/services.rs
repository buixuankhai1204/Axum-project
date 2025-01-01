use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::domain::employee::employee_department;
use crate::domain::employee::request::CreateNewEmployeeByUserUuidRequest;
use crate::domain::entity::{Employee, EmployeeDepartment, Organization, User};
use crate::domain::organization::Entity;
use crate::domain::user::ERoleUser;
use crate::domain::{employee, organization, user};
use crate::infrastructure::persistence::repo_interface::ReadRepository;
use axum::extract::State;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, IntoActiveModel, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_create_new_employee_by_user_exist(
    state: &AppState,
    req: &CreateNewEmployeeByUserUuidRequest,
) -> AppResult<i64> {
    info!("Register a new user request: {req:?}.");
    let tx = state.db.begin().await?;
    let organization = Organization::find_data_by_uuid(&tx, &req.organization_uuid).await;
    if organization.is_none() {
        return Err(AppError::EntityNotFoundError { entity: req.organization_uuid.to_string() });
    };

    let user = User::find_data_by_uuid(&tx, &req.user_uuid).await;
    if user.is_none() {
        return Err(AppError::EntityNotFoundError { entity: req.organization_uuid.to_string() });
    }

    let employee = employee::Model {
        id: Default::default(),
        employee_uuid: Uuid::new_v4(),
        user_id: user.unwrap().id,
        image_url: Default::default(),
        role: ERoleUser::Member,
        is_active: Default::default(),
        create_at: Utc::now().naive_utc(),
        update_at: Utc::now().naive_utc(),
    };
    let employee_insert = Employee::insert(employee.into_active_model()).exec(&tx).await?;
    if employee_insert.last_insert_id <= 0 {
        return Err(AppError::EntityNotAvailableError { entity: "Employee".to_string() });
    }

    let employee_department_model = employee_department::Model {
        employee_id: employee_insert.last_insert_id,
        department_id: organization.unwrap().id,
        create_at: Utc::now().naive_utc(),
        update_at: Utc::now().naive_utc(),
    };
    let employee_department_insert =
        EmployeeDepartment::insert(employee_department_model.into_active_model()).exec(&tx).await?;
    if employee_department_insert.last_insert_id.1 <= 0 {
        return Err(AppError::EntityNotAvailableError { entity: "Employee".to_string() });
    }
    tx.commit().await?;
    Ok(employee_insert.last_insert_id)
}
