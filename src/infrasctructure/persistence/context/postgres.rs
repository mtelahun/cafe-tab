use postgres_es::default_postgress_pool;
use secrecy::ExposeSecret;
use sqlx::{Pool, Postgres};

use super::connection_parameters::ConnectionParameters;

pub async fn postgres_pool(params: &ConnectionParameters) -> Pool<Postgres> {
    let connection_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        params.user,
        params.password.expose_secret(),
        params.host,
        params.port,
        params.db_name
    );

    default_postgress_pool(&connection_string).await
}

pub async fn create_db(conn_params: &ConnectionParameters) {
    let pool = sqlx::PgPool::connect(&conn_params.base_uri())
        .await
        .expect("sqlx failed to connect to PostgreSQL");
    let sql = format!(r#"CREATE DATABASE "{}";"#, conn_params.database_name());
    sqlx::query(&sql)
        .execute(&pool)
        .await
        .expect("failed to create the database");

    migrate_db(conn_params).await;
}

pub async fn migrate_db(conn_params: &ConnectionParameters) {
    let pool = sqlx::PgPool::connect(&format!(
        "{}/{}",
        conn_params.base_uri(),
        conn_params.database_name()
    ))
    .await
    .expect("failed to connect to newly created database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("database migration failed");
}
