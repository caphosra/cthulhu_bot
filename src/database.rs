use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

/// A user status.
#[derive(Default, sqlx::FromRow)]
pub struct Status {
    pub hp: i16,
    pub san: i16,
    pub mp: i16,
}

/// Represents a database for the bot.
///
/// A struct that controls database must inherit this trait.
#[serenity::async_trait]
pub trait BotDatabase {
    // Checks whether all of the features are ready.
    fn is_available(&self) -> bool;

    /// Tries to get the status. If failed, it returns `None`.
    async fn try_get_value(&self, id: u64) -> Option<Status>;

    /// Gets the status. If there is no record, it returns a default value.
    async fn get_value(&self, id: u64) -> Status;

    /// Reflects the changes of the status.
    async fn set_value(&self, id: u64, status: Status) -> Result<()>;
}

/// An abbreviation for `Box<dyn BotDatabase + Send + Sync>`.
pub type SizedBotDatabase = Box<dyn BotDatabase + Send + Sync>;

/// A database which contains nothing.
pub struct DummyDatabase;

#[serenity::async_trait]
impl BotDatabase for DummyDatabase {
    fn is_available(&self) -> bool {
        false
    }

    async fn try_get_value(&self, _id: u64) -> Option<Status> {
        panic!("This function is not implemented.")
    }

    async fn get_value(&self, _id: u64) -> Status {
        panic!("This function is not implemented.")
    }

    async fn set_value(&self, _id: u64, _status: Status) -> Result<()> {
        panic!("This function is not implemented.")
    }
}

/// A database which uses Postgres SQL.
pub struct PgDatabase {
    pool: Pool<Postgres>,
}

#[serenity::async_trait]
impl BotDatabase for PgDatabase {
    fn is_available(&self) -> bool {
        true
    }

    async fn try_get_value(&self, id: u64) -> Option<Status> {
        sqlx::query_as::<_, Status>("SELECT * FROM PlayerStatus WHERE id=$1")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await
            .ok()
    }

    async fn get_value(&self, id: u64) -> Status {
        match self.try_get_value(id).await {
            Some(status) => status,
            None => Status::default(),
        }
    }

    async fn set_value(&self, id: u64, status: Status) -> Result<()> {
        if self.try_get_value(id).await.is_some() {
            sqlx::query("UPDATE PlayerStatus SET hp=$1, san=$2, mp=$3 WHERE id=$4")
                .bind(status.hp)
                .bind(status.san)
                .bind(status.mp)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        } else {
            sqlx::query("INSERT INTO PlayerStatus VALUES ($1, $2, $3, $4)")
                .bind(id.to_string())
                .bind(status.hp)
                .bind(status.san)
                .bind(status.mp)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }
}

impl PgDatabase {
    /// Connects to a database.
    pub async fn init(uri: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(18)
            .connect(uri)
            .await?;

        println!("[BOT LOG] Connected to the database.");

        Ok(Self { pool })
    }
}
