pub type DbResult<T> = core::result::Result<T, DbErr>;

#[derive(Debug)]
pub enum DbErr {
    CanNotDo {
        error: String,
        collection: String,
        action: String,
    },
}

impl std::fmt::Display for DbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbErr::CanNotDo {
                error,
                collection,
                action,
            } => write!(
                f,
                "Can not do {} in {} bcs 😡{}😡",
                action, collection, error
            ),
        }
    }
}
