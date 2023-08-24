//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Serialize, Deserialize};
use super::super::utils::{generate, get_title_from_url};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "url")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)] // So that it stops looking for id in the json payload
    pub id: i32,
    pub url: String,
    #[serde(skip_deserializing)] // It should be generated when inserted
    pub title: String,
    pub slug: String,
    #[serde(skip_deserializing)] // Stop looking for created_at
    pub created_at: DateTime,
    #[serde(skip_deserializing)] // Stop looking for updated_at
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn before_save<'life0,'async_trait,C, >(self,_db: &'life0 C,insert:bool) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Self,DbErr> > + core::marker::Send+'async_trait> >where C:ConnectionTrait,C:'async_trait+ ,'life0:'async_trait,Self: core::marker::Send+'async_trait {
        
        Box::pin(async move {

            // Generate a randomised slug if not provided
            println!("Checking slug before insertion...");
            let slug = if (*self.slug.as_ref()).is_empty() {
                Set(generate(5))
            } else {
                self.slug
            };

            // Scrape for the url title
            let title = if self.title.is_not_set() {
                match get_title_from_url(self.url.as_ref()).await {
                    Ok(title) => {
                        println!("This is the url title: {title}");
                        Set(title)
                    },
                    Err(err) => {
                        // Handle the error from get_title_from_url appropriately
                        println!("Failed to fetch title: {:?}", err);
                        // You might want to return an error or a default title in case of failure
                       Set("default_str".to_string())
                    }
                }
            } else {
                self.title
            };

            let model = Self {
                url: self.url,
                slug,
                title,
                ..Default::default()
            };

            Ok(model)
        })
    }
}
