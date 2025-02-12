pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn setup_database() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create DB connection pool.")
}