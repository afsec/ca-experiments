// use super::Repository;
pub(crate) mod find_all;

pub(crate) struct BookRepo;

impl BookRepo {}

// #[async_trait]
// impl<'endpoint> Repository<'endpoint, Sqlite, (), ()> for BookRepo {
//     async fn run(&'endpoint self, db_conn_pool: &Pool<Sqlite>, input: ()) -> AppResult<()> {
//         Ok(())
//     }
// }
