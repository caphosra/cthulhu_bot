use sqlx::postgres::PgDatabaseError;
use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Default, sqlx::FromRow)]
pub struct Status {
    pub hp: i16,
    pub san: i16,
    pub mp: i16,
}

#[serenity::async_trait]
pub trait BotDatabase {
    async fn try_get_value(&self, id: u64) -> Option<Status>;
    async fn get_value(&self, id: u64) -> Status;
    async fn set_value(&self, id: u64, status: Status);
}

pub type SizedBotDatabase = Box<dyn BotDatabase + Send + Sync>;

pub struct DummyDatabase;

#[serenity::async_trait]
impl BotDatabase for DummyDatabase {
    async fn try_get_value(&self, id: u64) -> Option<Status> {
        None
    }

    async fn get_value(&self, id: u64) -> Status {
        Status::default()
    }

    async fn set_value(&self, id: u64, status: Status) {
        panic!("This function is not implemented.")
    }
}

pub struct PgDatabase {
    pool: Pool<Postgres>,
}

#[serenity::async_trait]
impl BotDatabase for PgDatabase {
    async fn try_get_value(&self, id: u64) -> Option<Status> {
        sqlx::query_as::<_, Status>("SELECT * FROM PlayerStatus WHERE id=?")
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

    async fn set_value(&self, id: u64, status: Status) {
        if self.try_get_value(id).await.is_some() {
            sqlx::query("UPDATE PlayerStatus SET hp=?, san=?, mp=? WHERE id=?")
                .bind(status.hp)
                .bind(status.san)
                .bind(status.mp)
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .expect("Cannot update the status.");
        } else {
            sqlx::query("INSERT INTO PlayerStatus VALUES (?, ?, ?, ?)")
                .bind(id.to_string())
                .bind(status.hp)
                .bind(status.san)
                .bind(status.mp)
                .execute(&self.pool)
                .await
                .expect("Cannot insert the status.");
        }
    }
}

impl PgDatabase {
    pub async fn init(uri: &str) -> Self {
        let pool = PgPool::connect(uri)
            .await
            .expect("Cannot connect to a sql.");

        Self { pool }
    }
}
