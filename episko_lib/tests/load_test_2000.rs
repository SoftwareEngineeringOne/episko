use common::generate_metadata;
use episko_lib::{database::DatabaseHandler, metadata::Metadata};
use sqlx::Row;
use sqlx::SqlitePool;
use uuid::Uuid;

mod common;

#[sqlx::test]
async fn test_read_write_2000(conn: SqlitePool) {
    const AMOUNT: usize = 5000;
    let db = DatabaseHandler::with_conn(conn);

    let mut ids: Vec<Uuid> = Vec::with_capacity(AMOUNT);
    for i in 0..AMOUNT {
        let data = generate_metadata(i);
        ids.push(data.id);
        data.write_to_db(&db).await.unwrap();
    }

    let row = sqlx::query("SELECT count(id) AS count FROM metadata")
        .fetch_one(db.conn())
        .await
        .unwrap();

    // assert AMOUNT entries have been created
    let count: u64 = row.try_get("count").unwrap();
    assert_eq!(AMOUNT, count as usize);

    for (i, id) in ids.into_iter().enumerate() {
        Metadata::from_db(&db, id)
            .await
            .expect(&format!("unable to retrieve a created element no. {i}"));
    }
}
