use crate::domain::entity::OrganizationEntity;
use crate::domain::organization;
use crate::infrastructure::persistence::repo_interface::ReadRepository;
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::domain::model::OrganizationModel;

#[async_trait::async_trait]
impl ReadRepository<OrganizationEntity> for OrganizationEntity {
    async fn find_data_by_id<DB>(conn: &DB, id: i64) -> Option<OrganizationModel>
    where DB: ConnectionTrait{
        let organization = OrganizationEntity::find_by_id::<i64>(id.into()).one(conn).await;
        if organization.is_err() {
            tracing::error!(
                "Something happen when query database: {:#?}.",
                organization.unwrap_err()
            );
            return None;
        };
        organization.unwrap_or_default()
    }

    async fn find_data_by_uuid<DB>(
        conn: &DB,
        uuid: &Uuid,
    ) -> Option<OrganizationModel>
    where DB: ConnectionTrait {
        let organization = OrganizationEntity::find()
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

    async fn find_all<DB>(conn: &DB) -> Option<Vec<OrganizationModel>>
    where DB: ConnectionTrait {
        todo!()
    }

    async fn find_data_by_name<DB>(
        conn: &DB,
        name: &str,
    ) -> Option<OrganizationModel>
    where DB: ConnectionTrait {
        todo!()
    }
}
