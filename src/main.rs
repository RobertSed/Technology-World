pub mod database;

fn main() {
    let pool = database::get_connection_pool();
    //let users = database::get_user(pool, "xx1x@xxx.com");
    let users= database::get_all_users(pool);
    println!("{:?}", users);
}
