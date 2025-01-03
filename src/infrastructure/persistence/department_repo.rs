use std::cell::RefCell;
use crate::core::error::{AppError, AppResult};
use crate::domain::entity::{DepartmentEntity, OrganizationEntity, PositionEntity};
use crate::domain::model::DepartmentModel;
use crate::domain::{department, organization, position};
use crate::infrastructure::persistence::repo_interface::{
    DeleteRepository, ReadRepository, WriteRepository,
};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, FromQueryResult, IntoActiveModel, QueryFilter};
use sea_orm::{ColumnTrait, EntityOrSelect, QuerySelect};
use uuid::Uuid;

#[async_trait]
impl ReadRepository<DepartmentEntity> for DepartmentEntity {
    async fn find_data_by_id<DB>(conn: &DB, id: i64) -> Option<DepartmentModel>
    where
        DB: ConnectionTrait,
    {
        let department = DepartmentEntity::find_by_id(id).one(conn).await;
        if department.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                department.unwrap_err()
            );
            return None;
        };
        department.unwrap_or_default()
    }

    async fn find_data_by_uuid<DB>(
        conn: &DB,
        uuid: &uuid::Uuid,
    ) -> Option<DepartmentModel>
    where
        DB: ConnectionTrait,
    {
        let department = DepartmentEntity::find()
            .filter(department::Column::DepartmentUuid.eq(*uuid))
            .one(conn)
            .await;
        if department.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                department.unwrap_err()
            );
            return None;
        };
        department.unwrap_or_default()
    }

    async fn find_all<DB>(conn: &DB) -> Option<Vec<DepartmentModel>>
    where
        DB: ConnectionTrait,
    {
        let departments = DepartmentEntity::find().all(conn).await;
        if departments.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                departments.unwrap_err()
            );
            return None;
        };
        Some(departments.unwrap_or_default())
    }

    async fn find_data_by_name<DB>(conn: &DB, name: &str) -> Option<DepartmentModel>
    where
        DB: ConnectionTrait,
    {
        let department =
            DepartmentEntity::find().filter(department::Column::Name.eq(name)).one(conn).await;
        if department.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                department.unwrap_err()
            );
            return None;
        };
        department.unwrap_or_default()
    }
}

// #[async_trait]
// impl WriteRepository<DepartmentEntity> for DepartmentEntity {
//     async fn create(conn: &DatabaseTransaction, model: &DepartmentModel) -> Option<i64> {
//         let department = DepartmentEntity::insert(model.clone().into_active_model()).exec(conn).await;
//         if department.is_err() {
//             tracing::error!(
//                 "Something happen when query database: {:#?}.",
//                 department.unwrap_err()
//             );
//             return None;
//         };
//         Some(department.unwrap().last_insert_id)
//     }
//
//     async fn update(conn: &DatabaseTransaction, model: DepartmentModel) -> Option<i64> {
//         let department_update = model.into_active_model().save(conn).await;
//         if department_update.is_err() {
//             tracing::error!(
//                 "Something happen when query database: {:#?}.",
//                 department_update.unwrap_err()
//             );
//             return None;
//         };
//
//         Some(department_update.unwrap().id.unwrap())
//     }
// }

#[async_trait]
impl DeleteRepository<DepartmentEntity> for DepartmentEntity {
    async fn delete_data(conn: &DatabaseTransaction, uuid: Uuid) -> Option<i64> {
        let department = DepartmentEntity::find()
            .filter(department::Column::DepartmentUuid.eq(uuid))
            .one(conn)
            .await;
        if department.is_err() {
            return None;
        }
        let mut department = department.unwrap().unwrap();
        department.is_active = Some(false);
        let department_delete = department.into_active_model().save(conn).await;
        if department_delete.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                department_delete.unwrap_err()
            );
            return None;
        }

        Some(department_delete.unwrap().id.unwrap())
    }
}

#[derive(Debug, FromQueryResult)]
struct Metadata {
    pub organization_id: i64,
    pub orgaization_name: String,
    pub department_id: i64,
    pub department_uuid: Uuid,
    pub department_name: String,
    pub position_id: i64,
    pub position_uuid: Uuid,
    pub position_name: String,
}
impl DepartmentEntity {
    pub async fn get_important_information(
        conn: &DatabaseTransaction,
        organization_uuid: Uuid,
    ) -> Option<Vec<Metadata>> {
        let select = DepartmentEntity::find()
            .filter(organization::Column::OrganizationUuid.eq(organization_uuid))
            .inner_join(PositionEntity)
            .inner_join(OrganizationEntity)
            .select_only()
            .column_as(organization::Column::OrganizationUuid, "organization_uuid")
            .column_as(organization::Column::Id, "organization_id")
            .column_as(organization::Column::Name, "organization_name")
            .column_as(department::Column::Id, "department_id")
            .column_as(department::Column::DepartmentUuid, "department_uuid")
            .column_as(department::Column::Name, "department_name")
            .column_as(position::Column::Id, "position_id")
            .column_as(position::Column::PositionUuid, "position_uuid")
            .column_as(position::Column::Name, "position_name");
        let results = select.into_model::<Metadata>().all(conn).await;
        if results.is_err() {
            tracing::error!("Something happen when query database: {:#?}.", results.unwrap_err());
            return None;
        }
        Some(results.unwrap_or_default())
    }
}
