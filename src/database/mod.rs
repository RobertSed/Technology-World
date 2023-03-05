pub mod models;
pub mod schema;

use diesel::result::Error;
use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, Pool};

use schema::users::dsl::*;

pub type MySQLPool = Pool<ConnectionManager<diesel::mysql::MysqlConnection>>;

pub fn get_connection_pool() -> MySQLPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let manager = ConnectionManager::<diesel::mysql::MysqlConnection>::new(database_url);
    Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .expect("Connection pool failed to build.")
}

pub fn get_all_users(pool: MySQLPool) -> Result<Vec<models::User>, Error> {
    let connection = &mut pool.get().expect("connection failed!");

    users.load::<models::User>(connection)
}

pub fn get_user(pool: MySQLPool, user_email: &'static str) -> Result<models::User, Error>{
    let connection = &mut pool.get().expect("connection failed!");

    users.filter(userEmail.eq(user_email.to_lowercase())).first::<models::User>(connection)
}