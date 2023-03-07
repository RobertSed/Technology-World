pub mod models;
pub mod schema;

use diesel::{r2d2::{self, ConnectionManager}, MysqlConnection};

pub type MySQLPool = r2d2::Pool<ConnectionManager<diesel::MysqlConnection>>;

/** This function generates a pool of connection to the database.
 
    returns the pool of connection or an error is failed to connect to the database.

*/
pub fn get_connection_pool() -> MySQLPool {
    let database_connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<diesel::MysqlConnection>::new(database_connection_url);
    r2d2::Pool::builder()
                        .build(manager)
                        .expect("Failed to connect to the database")
}