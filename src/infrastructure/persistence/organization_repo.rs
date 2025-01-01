use crate::domain::entity::Organization;
use crate::domain::organization;
use crate::infrastructure::persistence::repo_interface::ReadRepository;
use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

#[async_trait::async_trait]
impl ReadRepository<Organization> for Organization {
    async fn find_data_by_id(conn: &DatabaseTransaction, id: i64) -> Option<organization::Model> {
        let organization = Organization::find_by_id::<i64>(id.into()).one(conn).await;
        if organization.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                organization.unwrap_err()
            );
            return None;
        };
        organization.unwrap_or_default()
    }

    async fn find_data_by_uuid(
        conn: &DatabaseTransaction,
        uuid: &Uuid,
    ) -> Option<organization::Model> {
        let organization = Organization::find()
            .filter(organization::Column::OrganizationUuid.eq(*uuid))
            .one(conn)
            .await;
        if organization.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                organization.unwrap_err()
            );
            return None;
        };
        organization.unwrap_or_default()
    }

    async fn find_all(conn: &DatabaseTransaction) -> Option<Vec<organization::Model>> {
        todo!()
    }

    async fn find_data_by_name(
        conn: &DatabaseTransaction,
        name: &str,
    ) -> Option<organization::Model> {
        todo!()
    }
}
