pub mod models;
pub mod schema;

use std::env;

use diesel::{r2d2::{self, ConnectionManager}, MysqlConnection};

pub type MySQLPool = r2d2::Pool<ConnectionManager<diesel::MysqlConnection>>;

/** This function generates a pool of connection to the database.
 
    returns the pool of connection or an error is failed to connect to the database.

*/
pub fn get_connection_pool() -> MySQLPool {
    // Get database connection URL from enviroment variable.
    let database_connection_url = env::var("CLEARDB_DATABASE_URL").expect("No database URL set."); 

    // Create a manager to manage a pool of connections to the database.
    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<diesel::MysqlConnection>::new(database_connection_url);

    // Return the pool of connections
    r2d2::Pool::builder()
                        .build(manager)
                        .expect("Failed to connect to the database")
}