use r2d2_mysql::{
    mysql::{prelude::*, Opts, OptsBuilder},
    r2d2, MySqlConnectionManager,
};
use std::sync::Arc;

pub type PoolState = Arc<r2d2::Pool<MySqlConnectionManager>>;

pub struct StarrocksConnection {
    user: String,
    pass: Option<String>,
    host: String,
    port: u64,
    db: String,
}

impl StarrocksConnection {
    pub fn new(
        user: String,
        pass: Option<String>,
        host: String,
        port: u64,
        db: String,
    ) -> Result<StarrocksConnection, Box<dyn std::error::Error>> {
        Ok(StarrocksConnection {
            user,
            pass,
            host,
            port,
            db,
        })
    }

    pub fn connect(self) -> Result<PoolState, Box<dyn std::error::Error>> {
        let url = match self.pass {
            Some(password) => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.user, password, self.host, self.port, self.db
                )
            }
            None => {
                format!(
                    "mysql://{}@{}:{}/{}",
                    self.user, self.host, self.port, self.db
                )
            }
        };

        let opts = Opts::from_url(&url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MySqlConnectionManager::new(builder);
        let pool = Arc::new(r2d2::Pool::builder().max_size(5).build(manager).unwrap());

        Ok(pool)
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Arc<r2d2::Pool<MySqlConnectionManager>>,
}
