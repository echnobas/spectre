use deadpool_postgres::Client;
use tokio_postgres::Row;

pub type DatabaseResult<T> = Result<T, tokio_postgres::Error>;

pub struct User {
    rbx_id: i64,
    d_id: Option<i64>,
    xp: i64,
}

impl User {
    pub fn get_rbx_id(&self) -> i64 {
        self.rbx_id
    }

    pub fn get_d_id(&self) -> Option<i64> {
        self.d_id
    }

    pub fn get_xp(&self) -> i64 {
        self.xp
    }
}

impl From<Row> for User {
    fn from(r: Row) -> Self {
        Self {
            rbx_id: r.get("rbx_id"),
            d_id: r.get("d_id"),
            xp: r.get("xp"),
        }
    }
}

pub struct DatabaseClient {
    client: Client,
    guild_id: String,
}

impl DatabaseClient {
    pub async fn new<S: ToString>(
        pool: &deadpool_postgres::Pool,
        guild_id: S,
    ) -> Result<Self, deadpool_postgres::PoolError> {
        Ok(Self {
            client: pool.get().await?,
            guild_id: guild_id.to_string(),
        })
    }

    pub async fn get_version(&self) -> DatabaseResult<String> {
        self.client
            .query_one("SELECT version();", &[])
            .await
            .map(|r| r.get::<_, String>(0))
    }

    pub async fn register_group(&self) -> DatabaseResult<bool> {
        if self.client.query_opt("SELECT schema_name FROM information_schema.schemata WHERE schema_name = CONCAT('data_', $1::text);", &[ &self.guild_id ]  ).await?.is_some() {
            Ok(true)
        } else {
            self.client.execute("call register_group($1::text);", &[&self.guild_id]).await?;
            Ok(false)
        }
    }

    pub async fn get_member(&self, user_id: i64) -> DatabaseResult<Option<User>> {
        Ok(self.client.query_opt("SELECT * FROM GET_USER($1::text, $2::bigint) as t(rbx_id bigint, d_id bigint, xp bigint);", 
                        &[&self.guild_id, &user_id]).await?.map(|r| r.into()))
    }

    pub async fn add_xp(&self, user_id: i64, amount: i64) -> DatabaseResult<u64> {
        self.client
            .execute(
                "call ADD_XP($1::text, $2::bigint, $3::bigint)",
                &[&self.guild_id, &user_id, &amount],
            )
            .await
    }
}
