/// Application-specific business logic for our Library app.
// Save Author in our library
// Save Book in our library
// Fetch all books from our library
use crate::AppResult;

struct BookInteractor();

impl BookInteractor {
    pub(crate) fn save_book() -> AppResult<()> {
        Ok(())
    }
    pub(crate) fn fetch_all_books() -> AppResult<()> {
        Ok(())
    }
}

struct AuthorInteractor;

impl AuthorInteractor {
    pub(crate) fn save_author() -> AppResult<()> {
        Ok(())
    }
}
