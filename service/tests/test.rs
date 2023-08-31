mod prepare;

use entities::url;
use prepare::*;
use sea_orm::DatabaseConnection;
use service::{Mutation, Query};

#[tokio::test]
async fn main() {
    let db: &DatabaseConnection = &prepare_mock_db();

    {
        let url = Query::find_by_slug(db, "testing".to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(url.title, "Twitter".to_string());
        assert_eq!(url.id, 1);
    }

    {
        let urls = Query::find_all(db).await.unwrap();
        assert_eq!(urls.len(), 2);
    }
}
