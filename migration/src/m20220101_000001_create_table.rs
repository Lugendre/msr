use crate::sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        // gamesテーブルを作成
        conn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            r"CREATE TABLE IF NOT EXISTS games (
                     id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                     created_at TEXT NOT NULL,
                     updated_at TEXT NOT NULL,
                     name TEXT NOT NULL,
                     is_deleted BOOLEAN NOT NULL DEFAULT FALSE
            )",
        ))
        .await?;

        // artistsテーブルを作成
        conn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            r"CREATE TABLE IF NOT EXISTS artists (
                     id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                     created_at TEXT NOT NULL,
                     updated_at TEXT NOT NULL,
                     name TEXT NOT NULL,
                     is_deleted BOOLEAN NOT NULL DEFAULT FALSE
            )",
        ))
        .await?;

        // audio_formatsテーブルを作成
        conn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            r"CREATE TABLE IF NOT EXISTS audio_formats (
                     id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                     created_at TEXT NOT NULL,
                     updated_at TEXT NOT NULL,
                     format TEXT NOT NULL,
                     is_deleted BOOLEAN NOT NULL DEFAULT FALSE
            )",
        ))
        .await?;

        // albumsテーブルを作成
        conn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            r"CREATE TABLE IF NOT EXISTS albums (
                     id INTEGER NOT NULL PRIMARY KEY,
                     created_at TEXT NOT NULL,
                     updated_at TEXT NOT NULL,
                     name TEXT NOT NULL,
                     total_tracks INTEGER NOT NULL,
                     total_disks INTEGER NOT NULL,
                     cover_image_path TEXT NOT NULL,
                     game_id INTEGER NOT NULL,
                     artist_id INTEGER NOT NULL,
                     is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
                     CONSTRAINT fk_game_id
                         FOREIGN KEY (game_id)
                         REFERENCES games (id),
                     CONSTRAINT fk_artist_id
                         FOREIGN KEY (artist_id)
                         REFERENCES artists (id)
            )",
        ))
        .await?;

        // songsテーブルを作成
        conn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            r"CREATE TABLE IF NOT EXISTS songs (
                     id INTEGER NOT NULL PRIMARY KEY,
                     created_at TEXT NOT NULL,
                     updated_at TEXT NOT NULL,
                     name TEXT NOT NULL,
                     track_number INTEGER NOT NULL,
                     disk_number INTEGER NOT NULL,
                     source_path TEXT NOT NULL,
                     album_id INTEGER NOT NULL,
                     audio_format_id INTEGER NOT NULL,
                     artist_id INTEGER NOT NULL,
                     is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
                     CONSTRAINT fk_album_id
                         FOREIGN KEY (album_id)
                         REFERENCES albums (id),
                     CONSTRAINT audio_format_id
                         FOREIGN KEY (audio_format_id)
                         REFERENCES audio_formats (id),
                     CONSTRAINT fk_artist_id
                         FOREIGN KEY (artist_id)
                         REFERENCES artists (id)
            )",
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            r"DROP TABLE IF EXISTS
                     songs,
                     albums,
                     audio_formats,
                     artists,
                     games;
                  ",
        ))
        .await?;

        Ok(())
    }
}
