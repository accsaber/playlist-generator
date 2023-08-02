use crate::config::Config;
use mysql_async::Pool;
use std::sync::Arc;

pub type Db = Arc<mysql_async::Pool>;

pub async fn connect(config: &Config) -> mysql_async::Result<Db> {
    let pool = Pool::new(config.connection_string.as_str());

    Ok(Arc::new(pool))
}
