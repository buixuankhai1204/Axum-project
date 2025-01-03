use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::domain::department::Column;
use crate::domain::employee::employee_department::ActiveModel;
use crate::domain::employee::employee_position::ActiveModel as ActiveModelPosition;
use crate::domain::employee::request::{
    CreateNewEmployeeByUserUuidRequest, CreateNewEmployeeRequest, DeleteEmployeeRequest,
    UpdateEmployeeRequest,
};
use crate::domain::employee::{employee_department, employee_position};
use crate::domain::entity::{
    DepartmentEntity, EmployeeDepartmentEntity, EmployeeEntity, EmployeePositionEntity,
    OrganizationEntity, PositionEntity, UserEntity,
};
use crate::domain::model::{EmployeeDepartmentModel, EmployeePositionModel};
use crate::domain::organization::Entity;
use crate::domain::{department, employee, organization, position, user};
use crate::infrastructure::persistence::repo_interface::{
    DeleteRepository, ReadRepository, WriteRepository,
};
use axum::extract::State;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    IntoActiveModel, NotSet, QueryFilter, TransactionTrait,
};
use std::cell::RefCell;
use tracing::info;
use uuid::Uuid;

pub async fn service_create_new_employee_by_user_exist(
    state: &AppState,
    req: &CreateNewEmployeeByUserUuidRequest,
) -> AppResult<i64> {
    info!("Register a new user request: {req:?}.");
    let tx = state.db.begin().await?;
    let organization = OrganizationEntity::find_data_by_uuid(&tx, &req.organization_uuid).await;
    if organization.is_none() {
        return Err(AppError::EntityNotFoundError { detail: req.organization_uuid.to_string() });
    };
    let new_employee_id = insert_new_employee(&tx, req).await;
    if new_employee_id.is_err() {
        return Err(new_employee_id.unwrap_err());
    }
    tx.commit().await?;
    new_employee_id
}

pub async fn insert_new_employee(
    tx: &DatabaseTransaction,
    create_new_employee_by_user_uuid_request: &CreateNewEmployeeByUserUuidRequest,
) -> AppResult<i64> {
    let user =
        UserEntity::find_data_by_uuid(tx, &create_new_employee_by_user_uuid_request.user_uuid)
            .await;
    if user.is_none() {
        return Err(AppError::EntityNotFoundError {
            detail: create_new_employee_by_user_uuid_request.organization_uuid.to_string(),
        });
    }

    let employee = employee::ActiveModel {
        id: NotSet,
        employee_uuid: Set(Uuid::new_v4()),
        user_id: Set(user.unwrap().id),
        image_url: Set("image.jpg".to_string()),
        status: Default::default(),
        create_at: Set(Utc::now().naive_utc()),
        update_at: Set(Utc::now().naive_utc()),
    };
    let employee_insert = EmployeeEntity::insert(employee).exec(tx).await;
    println!("employee_insert: {:?}", employee_insert);
    if employee_insert.is_err() {
        return Err(AppError::EntityNotAvailableError { detail: "employee_insert".to_string() });
    }
    let department = DepartmentEntity::find_data_by_uuid(
        tx,
        &create_new_employee_by_user_uuid_request.department_uuid,
    )
    .await;
    if department.is_none() {
        return Err(AppError::EntityNotFoundError {
            detail: create_new_employee_by_user_uuid_request.department_uuid.to_string(),
        });
    };
    let employee_insert_id = employee_insert.unwrap().last_insert_id;
    let employee_department_model = employee_department::Model {
        employee_id: employee_insert_id,
        department_id: department.unwrap().id,
        create_at: Utc::now().naive_utc(),
        update_at: Utc::now().naive_utc(),
    };
    let employee_department_insert =
        EmployeeDepartmentEntity::insert(employee_department_model.into_active_model())
            .exec(tx)
            .await?;
    if employee_department_insert.last_insert_id.0 < 0 {
        return Err(AppError::EntityNotAvailableError {
            detail: "employee_department_insert".to_string(),
        });
    }

    let position = PositionEntity::find_data_by_uuid(
        tx,
        &create_new_employee_by_user_uuid_request.position_uuid,
    )
    .await;
    if position.is_none() {
        return Err(AppError::EntityNotFoundError {
            detail: create_new_employee_by_user_uuid_request.position_uuid.to_string(),
        });
    };
    let employee_position_model = employee_position::Model {
        employee_id: employee_insert_id,
        position_id: position.unwrap().id,
        create_at: Utc::now().naive_utc(),
        update_at: Utc::now().naive_utc(),
    };

    let employee_position_insert =
        EmployeePositionEntity::insert(employee_position_model.into_active_model()).exec(tx).await;
    if employee_position_insert.is_err() {
        return Err(AppError::EntityNotAvailableError {
            detail: "employee_position_insert".to_string(),
        });
    }

    Ok(employee_insert_id)
}

pub async fn service_create_new_employee(
    state: &AppState,
    req: &CreateNewEmployeeRequest,
) -> AppResult<i64> {
    let tx = state.db.begin().await?;
    let organization = OrganizationEntity::find_data_by_uuid(&tx, &req.organization_uuid).await;
    if organization.is_none() {
        return Err(AppError::EntityNotFoundError { detail: req.organization_uuid.to_string() });
    };

    let mut user_model = user::Model {
        id: Default::default(),
        user_uuid: Default::default(),
        creator_id: 1,
        role_id: 0,
        full_name: req.get_fullname().to_string(),
        password: "ABCDEFGH".to_string(),
        email: req.get_email().to_string(),
        gender: *req.get_gender(),
        phone_number: None,
        address: req.get_address().to_owned(),
        picture: None,
        language: None,
        status: 1,
        last_login: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),

        deleted_at: None,
    };
    let user_insert_id = UserEntity::create(&tx, &user_model).await;
    if user_insert_id.is_none() {
        return Err(AppError::EntityNotAvailableError { detail: "User".to_string() });
    }

    let user = UserEntity::find_data_by_id(&tx, user_insert_id.unwrap()).await;
    if user.is_none() {
        return Err(AppError::EntityNotFoundError { detail: "fail".to_string() });
    }

    let create_new_employee_by_user_exist = CreateNewEmployeeByUserUuidRequest {
        user_uuid: user.unwrap().user_uuid,
        organization_uuid: req.organization_uuid,
        department_uuid: req.department_uuid,
        position_uuid: req.position_uuid,
    };
    let employee_insert_id = insert_new_employee(&tx, &create_new_employee_by_user_exist).await;
    if employee_insert_id.is_err() {
        return Err(employee_insert_id.unwrap_err());
    };

    tx.commit().await?;
    employee_insert_id
}

pub async fn service_update_employee(
    state: &AppState,
    req: &UpdateEmployeeRequest,
) -> AppResult<i64> {
    let tx = state.db.begin().await?;
    let employee = EmployeeEntity::find_data_by_uuid(&tx, &req.get_employee_uuid()).await;
    if employee.is_none() {
        return Err(AppError::EntityNotFoundError { detail: "".to_string() });
    }

    let mut employee_active = employee.unwrap();
    let employee_department_delete = EmployeeDepartmentEntity::delete_many()
        .filter(employee_department::Column::EmployeeId.eq(employee_active.id))
        .exec(&tx)
        .await;
    if employee_department_delete.is_err() {
        return Err(AppError::EntityNotAvailableError { detail: "Employee".to_string() });
    }

    let employee_position_delete = EmployeePositionEntity::delete_many()
        .filter(employee_position::Column::EmployeeId.eq(employee_active.id))
        .exec(&tx)
        .await;
    if employee_position_delete.is_err() {
        return Err(AppError::EntityNotAvailableError { detail: "Employee".to_string() });
    }

    if req.get_department_uuid().is_some() {
        let departments = DepartmentEntity::find()
            .filter(department::Column::DepartmentUuid.is_in(req.get_department_uuid().unwrap()))
            .all(&tx)
            .await;
        if departments.is_err() {
            return Err(AppError::EntityNotAvailableError { detail: "Employee".to_string() });
        }
        let employee_department_insert = EmployeeDepartmentEntity::insert_many(
            departments?
                .iter()
                .map(|employee_department_id| {
                    EmployeeDepartmentModel {
                        employee_id: employee_active.id,
                        department_id: employee_department_id.id,
                        create_at: Utc::now().naive_utc(),
                        update_at: Utc::now().naive_utc(),
                    }
                    .into_active_model()
                })
                .collect::<Vec<ActiveModel>>(),
        )
        .exec(&tx)
        .await;
        if employee_department_insert.is_err() {
            return Err(AppError::InvalidPayloadError { 0: "".to_string() });
        }
    }

    if req.get_position_uuid().is_some() {
        let positions = PositionEntity::find()
            .filter(position::Column::PositionUuid.is_in(req.get_position_uuid().unwrap()))
            .all(&tx)
            .await;
        if positions.is_err() {
            return Err(AppError::EntityNotAvailableError { detail: "Employee".to_string() });
        }

        let employee_position_insert = EmployeePositionEntity::insert_many(
            positions?
                .iter()
                .map(|employee_position_id| {
                    EmployeePositionModel {
                        employee_id: employee_active.id,
                        position_id: employee_position_id.id,
                        create_at: Utc::now().naive_utc(),
                        update_at: Utc::now().naive_utc(),
                    }
                    .into_active_model()
                })
                .collect::<Vec<ActiveModelPosition>>(),
        )
        .exec(&tx)
        .await;
        if employee_position_insert.is_err() {
            return Err(AppError::InvalidPayloadError { 0: "".to_string() });
        };
    }

    employee_active.update_at = Utc::now().naive_utc();
    let data = employee_active.into_active_model().save(&tx).await;
    if data.is_err() {
        return Err(AppError::EntityNotAvailableError { detail: "Employee".to_string() });
    }

    tx.commit().await?;
    Ok(data?.id.unwrap())
}

pub async fn service_delete_employee(
    state: &AppState,
    req: &DeleteEmployeeRequest,
) -> AppResult<i64> {
    let tx = state.db.begin().await?;
    let employee_delete = EmployeeEntity::delete_data(&tx, req.get_employee_uuid()).await;
    if employee_delete.is_none() {
        return Err(AppError::BadRequestError("Can not delete employee".to_string()));
    }

    tx.commit().await?;
    Ok(employee_delete.unwrap())
}
