use diesel::r2d2::ConnectionManager;
use diesel::mysql::MysqlConnection;

pub type Conn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_database_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("db pool build error occurred")
}
