use crate::domain::entity::{DepartmentEntity, EmployeeEntity};
use crate::domain::model::EmployeeModel;
use crate::domain::{department, employee};
use crate::infrastructure::persistence::repo_interface::{
    DeleteRepository, ReadRepository, WriteRepository,
};
use crate::util::filter_and_pagination::{sort_and_paginate, EModule, PageQueryParam};
use async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseTransaction,
    EntityTrait, IntoActiveModel, QueryFilter, QueryTrait,
};
use std::cell::RefCell;
use std::fmt::Debug;
use uuid::Uuid;

#[async_trait]
impl ReadRepository<EmployeeEntity> for EmployeeEntity {
    async fn find_data_by_id<DB>(conn: &DB, id: i64) -> Option<EmployeeModel>
    where
        DB: ConnectionTrait + Debug,
    {
        let user = EmployeeEntity::find_by_id(id).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }

    async fn find_data_by_uuid<DB>(conn: &DB, uuid: &uuid::Uuid) -> Option<EmployeeModel>
    where
        DB: ConnectionTrait + Debug,
    {
        let user =
            EmployeeEntity::find().filter(employee::Column::EmployeeUuid.eq(*uuid)).one(conn).await;
        if user.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
            return None;
        };
        user.unwrap_or_default()
    }

    async fn find_all<DB>(conn: &DB, query_params: PageQueryParam) -> Option<Vec<EmployeeModel>>
    where
        DB: ConnectionTrait + Debug,
    {
        let employees = sort_and_paginate(
            conn,
            &mut EmployeeEntity::find(),
            query_params,
            EModule::Employee,
        ).await;
        if employees.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", employees.unwrap_err());
            return None;
        };
        Some(employees.unwrap_or_default())
    }

    async fn find_data_by_name<DB>(conn: &DB, name: &str) -> Option<EmployeeModel>
    where
        DB: ConnectionTrait + Debug,
    {
        todo!("Dont need to implement this method.")
    }
}

// #[async_trait]
// impl WriteRepository<EmployeeEntity> for EmployeeEntity {
//     async fn create(conn: &DatabaseTransaction, model: &EmployeeModel) -> Option<i64> {
//         let user = EmployeeEntity::insert(model.clone().into_active_model()).exec(conn).await;
//         if user.is_err() {
//             tracing::error!("Something happen when query database: {:#?}.", user.unwrap_err());
//             return None;
//         };
//         Some(user.unwrap().last_insert_id)
//     }
//
//     async fn update(conn: &DatabaseTransaction, model: EmployeeModel) -> Option<i64> {
//         let user_update = model.into_active_model().save(conn).await;
//         if user_update.is_err() {
//             tracing::error!(
//                 "Something happen when query database: {:#?}.",
//                 user_update.unwrap_err()
//             );
//             return None;
//         };
//
//         Some(user_update.unwrap().id.unwrap())
//     }
// }

#[async_trait]
impl DeleteRepository<EmployeeEntity> for EmployeeEntity {
    async fn delete_data(conn: &DatabaseTransaction, uuid: Uuid) -> Option<i64> {
        let employee_data =
            EmployeeEntity::find().filter(employee::Column::EmployeeUuid.eq(uuid)).one(conn).await;
        if employee_data.is_err() {
            return None;
        }

        let mut employee = match employee_data.unwrap() {
            Some(data) => data.into_active_model(),
            None => return None,
        };

        employee.status = Set(Some(0));
        let employee_delete = employee.save(conn).await;
        if employee_delete.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                employee_delete.unwrap_err()
            );
            return None;
        }

        Some(employee_delete.unwrap().id.unwrap())
    }
}
