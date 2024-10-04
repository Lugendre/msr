use itertools::Itertools;
use sea_orm::{
    AccessMode, ConnectionTrait, DatabaseBackend, DatabaseConnection, DatabaseTransaction, DbErr,
    ExecResult, QueryResult, Statement, TransactionError, TransactionTrait,
};
use std::future::Future;
use std::pin::Pin;

pub trait ProvideDatabase {
    fn provide_database(&self) -> &DatabaseConnection;
}

impl ProvideDatabase for DatabaseConnection {
    fn provide_database(&self) -> &DatabaseConnection {
        self
    }
}

pub async fn read_only_transaction<R, F, T, E>(
    conn: &R,
    callback: F,
) -> Result<T, TransactionError<E>>
where
    R: ProvideDatabase,
    E: std::error::Error + Send,
    T: Send,
    F: for<'c> FnOnce(
            &'c sea_orm::DatabaseTransaction,
        ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
        + Send,
{
    conn.provide_database()
        .transaction_with_config(callback, None, Some(AccessMode::ReadOnly))
        .await
}

pub async fn read_write_transaction<R, F, T, E>(
    conn: &R,
    callback: F,
) -> Result<T, TransactionError<E>>
where
    R: ProvideDatabase,
    E: std::error::Error + Send,
    T: Send,
    F: for<'c> FnOnce(
            &'c sea_orm::DatabaseTransaction,
        ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
        + Send,
{
    conn.provide_database()
        .transaction_with_config(callback, None, Some(AccessMode::ReadWrite))
        .await
}

pub async fn query_all<T: Into<String>>(
    tx: &DatabaseTransaction,
    sql: T,
) -> Result<Vec<QueryResult>, DbErr> {
    tx.query_all(Statement::from_string(DatabaseBackend::Sqlite, sql))
        .await
}

pub async fn query_all_and_values<T: Into<String>, I>(
    tx: &DatabaseTransaction,
    sql: T,
    values: I,
) -> Result<Vec<QueryResult>, DbErr>
where
    I: IntoIterator<Item = sea_orm::Value>,
{
    tx.query_all(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        sql,
        values,
    ))
    .await
}

pub async fn query_one<T: Into<String>>(
    tx: &DatabaseTransaction,
    sql: T,
) -> Result<Option<QueryResult>, DbErr> {
    tx.query_one(Statement::from_string(DatabaseBackend::Sqlite, sql))
        .await
}

pub async fn query_one_and_values<T: Into<String>, I>(
    tx: &DatabaseTransaction,
    sql: T,
    values: I,
) -> Result<Option<QueryResult>, DbErr>
where
    I: IntoIterator<Item = sea_orm::Value>,
{
    tx.query_one(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        sql,
        values,
    ))
    .await
}

pub async fn execute<T: Into<String>>(
    tx: &DatabaseTransaction,
    sql: T,
) -> Result<ExecResult, DbErr> {
    tx.execute(Statement::from_string(DatabaseBackend::Sqlite, sql))
        .await
}

pub async fn execute_and_values<T: Into<String>, I>(
    tx: &DatabaseTransaction,
    sql: T,
    values: I,
) -> Result<ExecResult, DbErr>
where
    I: IntoIterator<Item = sea_orm::Value>,
{
    tx.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        sql,
        values,
    ))
    .await
}

pub async fn multiple_delete<T: Into<String>, I>(
    tx: &DatabaseTransaction,
    sql: T,
    params: I,
) -> Result<Option<ExecResult>, DbErr>
where
    I: IntoIterator<Item = sea_orm::Value>,
{
    let params = params.into_iter().collect::<Vec<_>>();
    if params.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            tx.execute(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                sql.into().replace(
                    "(?)",
                    format!("({})", &vec!["?"; params.len()].iter().join(", ")).as_str(),
                ),
                params,
            ))
            .await?,
        ))
    }
}
