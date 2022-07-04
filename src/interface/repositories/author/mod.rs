// use super::Repository;
pub(crate) mod find_all;

pub(crate) struct AuthorRepo;

impl AuthorRepo {}

// #[async_trait]
// impl<'endpoint> Repository<'endpoint, Sqlite, (), ()> for AuthorRepo {
//     async fn run(&'endpoint self, db_conn_pool: &Pool<Sqlite>, input: ()) -> AppResult<()> {
//         Ok(())
//     }
// }
