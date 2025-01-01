use crate::core::error::{AppError, AppResult};
use crate::domain::{employee, user};
use crate::infrastructure::migrations::SimpleExpr;
use sea_orm::{
    entity, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, QueryOrder, Select,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(
    Serialize, Deserialize, Debug, Display, ToSchema, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Direction {
    DESC,
    ASC,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams, Clone)]
pub struct PageQueryParam {
    pub page_num: u64,
    pub page_size: u64,
    pub sort_by: Option<String>,
    pub sort_direction: Option<Direction>,
    #[param(allow_reserved, example = json!("column:eq:value"))]
    pub filter: String,
}

enum Either<A, B> {
    UserColumn(A),
    EmployeeColumn(B),
}

pub fn get_simple_expression(query_string: &str, module_name: EModule) -> Option<SimpleExpr> {
    let parts = query_string.split(':').collect::<Vec<_>>();
    if parts.len() != 3 {
        return None;
    }

    let column: Either<user::Column, employee::Column> = match module_name {
        EModule::User => match parts[0] {
            "username" => Either::UserColumn(user::Column::Username),
            "email" => Either::UserColumn(user::Column::Email),
            "id" => Either::UserColumn(user::Column::Id),
            _ => return None,
        },
        EModule::Employee => match parts[0] {
            "role" => Either::EmployeeColumn(employee::Column::Role),
            "userId" => Either::EmployeeColumn(employee::Column::UserId),
            _ => return None,
        },
        _ => return None,
    };

    match column {
        Either::UserColumn(user) => match parts[1] {
            "eq" => Some(user.eq(parts[2])),
            "lte" => Some(user.lte(parts[2].parse::<i32>().unwrap())),
            "lt" => Some(user.lt(parts[2].parse::<i32>().unwrap())),
            "ne" => Some(user.ne(parts[2])),
            "gte" => Some(user.gte(parts[2].parse::<i32>().unwrap())),
            "gt" => Some(user.gt(parts[2].parse::<i32>().unwrap())),
            "contains" => Some(user.contains(parts[2])),
            "null" => Some(user.if_null(parts[2])),
            _ => return None,
        },
        Either::EmployeeColumn(employee) => Some(employee.eq(parts[2])),
        _ => None,
    }
}

#[derive(Clone, Copy)]
pub enum EModule {
    User,
    Employee,
}

pub async fn sort_and_paginate<E, M>(
    conn: &DatabaseConnection,
    select: &mut Select<E>,
    param: PageQueryParam,
    module_name: EModule,
) -> AppResult<Vec<M>>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    let filters = param.filter.split(',').collect::<Vec<_>>();
    let mut select_clone_object = select.clone();
    for item in filters {
        let columns = get_simple_expression(item, module_name);
        let columns_data = match columns {
            None => {
                return Err(AppError::BadRequestError(
                    "query string not correct format".to_string(),
                ));
            },
            Some(value) => value,
        };

        select_clone_object = select_clone_object.filter(columns_data);
    }

    select_clone_object = match param.sort_direction {
        Some(Direction::DESC) => select_clone_object.order_by_desc(user::Column::CreateAt),
        _ => select_clone_object.order_by_asc(user::Column::CreateAt),
    };

    let models: Vec<M> =
        select_clone_object.paginate(conn, param.page_size).fetch_page(param.page_num).await?;
    Ok(models)
}
