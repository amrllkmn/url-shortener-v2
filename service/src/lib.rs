use sea_orm::*;

use entities::{prelude::Url, url};
pub struct Mutation;

pub struct Query;

impl Mutation {
    pub async fn create(db: &DbConn, form_data: url::Model) -> Result<url::Model, DbErr> {
        url::ActiveModel {
            url: Set(form_data.url.to_owned()),
            slug: Set(form_data.slug.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        if let Some(url) = Url::find_by_id(id).one(db).await? {
            url.delete(db).await
        } else {
            Err(DbErr::Custom("Cannot find URL.".to_string()))
        }
    }
}

impl Query {
    pub async fn find_all(db: &DbConn) -> Result<Vec<url::Model>, DbErr> {
        Url::find().all(db).await
    }

    pub async fn find_by_slug(db: &DbConn, slug: String) -> Result<Option<url::Model>, DbErr> {
        Url::find().filter(url::Column::Slug.eq(slug)).one(db).await
    }
}
