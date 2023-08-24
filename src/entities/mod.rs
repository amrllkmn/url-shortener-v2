//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2
use sea_orm::*;

pub mod prelude;

pub mod url;

pub struct Mutation;

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
}
