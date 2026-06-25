use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use elytra_core::types::{AvailablePlugin, PendingPlugin, Plugin};

/// Wraps the Postgres connection pool and all plugin-related queries.
///
/// This is the single place that knows about SQL. Everything outside this
/// file should go through `Database`, not `sqlx`, directly.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("a plugin with kebabbed name '{0}' already exists")]
    DuplicateKebabbedName(String),

    #[error("no pending plugin found with id {0}")]
    PendingPluginNotFound(i64),
}

impl Database {
    /// Connects to Postgres and runs migrations.
    ///
    /// `database_url` looks like: `postgres://user:pass@localhost/elytra`
    pub async fn connect(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        let db = Self { pool };
        db.migrate().await?;
        Ok(db)
    }

    /// Creates a `Database` from an existing pool, e.g. for testing.
    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates the tables if they don't already exist.
    ///
    /// This is intentionally simple (no migration framework) since the
    /// schema is small right now. If it grows, swap this for `sqlx::migrate!`.
    async fn migrate(&self) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS pending_plugins (
                id BIGSERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                kebabbed_name TEXT NOT NULL UNIQUE,
                repository_url TEXT NOT NULL,
                submitted_at TIMESTAMPTZ NOT NULL DEFAULT now()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS available_plugins (
                id BIGSERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                kebabbed_name TEXT NOT NULL UNIQUE,
                repository_url TEXT NOT NULL,
                releases_url TEXT,
                approved_at TIMESTAMPTZ NOT NULL DEFAULT now()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ---- pending plugins ----------------------------------------------

    /// Inserts a new pending plugin and returns it with its assigned id.
    pub async fn insert_pending_plugin(
        &self,
        plugin: &PendingPlugin,
    ) -> Result<i64, DatabaseError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO pending_plugins (name, kebabbed_name, repository_url)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(plugin.name())
        .bind(plugin.kebabbed_name())
        .bind(plugin.repository_url())
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(id) => Ok(id),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => Err(
                DatabaseError::DuplicateKebabbedName(plugin.kebabbed_name().to_owned()),
            ),
            Err(e) => Err(e.into()),
        }
    }

    /// Lists all pending plugins, oldest submission first (the approval queue).
    pub async fn list_pending_plugins(&self) -> Result<Vec<PendingPluginRow>, DatabaseError> {
        let rows = sqlx::query_as::<_, PendingPluginRow>(
            r#"
            SELECT id, name, kebabbed_name, repository_url, submitted_at
            FROM pending_plugins
            ORDER BY submitted_at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /// Approves a pending plugin: creates the corresponding `available_plugins`
    /// row and removes the pending one, atomically. Returns the new `AvailablePlugin`.
    pub async fn approve_pending_plugin(
        &self,
        pending_id: i64,
    ) -> Result<AvailablePlugin, DatabaseError> {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        let pending = sqlx::query_as::<_, PendingPluginRow>(
            r#"
            SELECT id, name, kebabbed_name, repository_url, submitted_at
            FROM pending_plugins
            WHERE id = $1
            "#,
        )
        .bind(pending_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(DatabaseError::PendingPluginNotFound(pending_id))?;

        let new_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO available_plugins (name, kebabbed_name, repository_url)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(&pending.name)
        .bind(&pending.kebabbed_name)
        .bind(&pending.repository_url)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM pending_plugins WHERE id = $1")
            .bind(pending_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(AvailablePlugin::new(
            &pending.name,
            &pending.repository_url,
            new_id as u64,
        ))
    }

    /// Rejects (deletes) a pending plugin without approving it.
    pub async fn reject_pending_plugin(&self, pending_id: i64) -> Result<(), DatabaseError> {
        let result = sqlx::query("DELETE FROM pending_plugins WHERE id = $1")
            .bind(pending_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::PendingPluginNotFound(pending_id));
        }

        Ok(())
    }

    // ---- available plugins ---------------------------------------------

    /// Fetches an available plugin by its database id.
    pub async fn get_available_plugin(
        &self,
        id: i64,
    ) -> Result<Option<AvailablePlugin>, DatabaseError> {
        let row = sqlx::query_as::<_, AvailablePluginRow>(
            r#"
            SELECT id, name, kebabbed_name, repository_url, releases_url, approved_at
            FROM available_plugins
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    /// Fetches an available plugin by its kebabbed name (e.g. for URL lookups).
    pub async fn get_available_plugin_by_kebabbed_name(
        &self,
        kebabbed_name: &str,
    ) -> Result<Option<AvailablePlugin>, DatabaseError> {
        let row = sqlx::query_as::<_, AvailablePluginRow>(
            r#"
            SELECT id, name, kebabbed_name, repository_url, releases_url, approved_at
            FROM available_plugins
            WHERE kebabbed_name = $1
            "#,
        )
        .bind(kebabbed_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    /// Lists all available plugins.
    pub async fn list_available_plugins(&self) -> Result<Vec<AvailablePlugin>, DatabaseError> {
        let rows = sqlx::query_as::<_, AvailablePluginRow>(
            r#"
            SELECT id, name, kebabbed_name, repository_url, releases_url, approved_at
            FROM available_plugins
            ORDER BY approved_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Updates the releases_url for an available plugin.
    pub async fn set_releases_url(
        &self,
        id: i64,
        releases_url: Option<&str>,
    ) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE available_plugins SET releases_url = $1 WHERE id = $2")
            .bind(releases_url)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Raw row shape for `pending_plugins`. Kept separate from `PendingPlugin`
/// since the in-memory type has no `id` field, but the DB row needs one.
#[derive(sqlx::FromRow)]
pub struct PendingPluginRow {
    pub id: i64,
    pub name: String,
    pub kebabbed_name: String,
    pub repository_url: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

/// Raw row shape for `available_plugins`, converted into `AvailablePlugin`.
#[derive(sqlx::FromRow)]
struct AvailablePluginRow {
    id: i64,
    name: String,
    #[allow(dead_code)] // read from DB but AvailablePlugin re-derives this from `name`
    kebabbed_name: String,
    repository_url: String,
    releases_url: Option<String>,
    #[allow(dead_code)] // not currently exposed on AvailablePlugin
    approved_at: chrono::DateTime<chrono::Utc>,
}

impl From<AvailablePluginRow> for AvailablePlugin {
    fn from(row: AvailablePluginRow) -> Self {
        let mut plugin = AvailablePlugin::new(&row.name, &row.repository_url, row.id as u64);
        if let Some(url) = row.releases_url {
            plugin.set_releases_url(url.into_boxed_str());
        }
        plugin
    }
}