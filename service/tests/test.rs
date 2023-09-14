mod prepare;

use chrono::Utc;
use entities::url;
use prepare::*;
use sea_orm::DatabaseConnection;
use service::{Mutation, Query};

#[tokio::test]
async fn main() {
    let db: &DatabaseConnection = &prepare_mock_db();

    // Find by slug
    {
        let url = Query::find_by_slug(db, "testing".to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(url.title, "Twitter".to_string());
        assert_eq!(url.id, 1);
    }

    // Find all
    {
        let urls = Query::find_all(db).await.unwrap();
        assert_eq!(urls.len(), 2);
    }

    // Create a URL
    {
        let time = Utc::now().naive_utc();
        let url = Mutation::create(
            db,
            url::Model {
                id: 3,
                title: "Google".to_string(),
                url: "https://google.com".to_string(),
                slug: "testing".to_string(),
                created_at: time,
                updated_at: time,
            },
        )
        .await
        .unwrap();
        assert_eq!(url.id, 3);
    }

    // Delete URL
    {
        let result = Mutation::delete(db, 4).await.unwrap();
        assert_eq!(result.rows_affected, 1);
    }
}
