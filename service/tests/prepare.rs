use ::entities::url;
use sea_orm::*;

// Parse the input string into a Naive

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    // Create a NaiveDate
    let date = NaiveDate::from_ymd_opt(2023, 8, 31).expect("Invalid date components");

    // Create a NaiveTime
    let time = NaiveTime::from_hms_opt(15, 30, 0).expect("Invalid time components");

    // Combine the NaiveDate and NaiveTime to create a NaiveDateTime
    let datetime = NaiveDateTime::new(date, time);
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[url::Model {
            id: 1,
            url: "https://twitter.com".to_string(),
            title: "Twitter".to_string(),
            slug: "testing".to_string(),
            created_at: datetime,
            updated_at: datetime,
        }]])
        .append_query_results([[
            url::Model {
                id: 1,
                url: "https://twitter.com".to_string(),
                title: "Twitter".to_string(),
                slug: "testing".to_string(),
                created_at: datetime,
                updated_at: datetime,
            },
            url::Model {
                id: 2,
                url: "https://youtube.com".to_string(),
                title: "youtube".to_string(),
                slug: "youtube".to_string(),
                created_at: datetime,
                updated_at: datetime,
            },
        ]])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 14,
                rows_affected: 4,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 5,
            },
        ])
        .into_connection()
}
