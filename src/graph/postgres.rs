use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> PostgresPool {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let mgr = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(mgr).expect("Couldn't build PG pool!")
}
