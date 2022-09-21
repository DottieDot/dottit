use super::RepositoryError;

pub type RepositoryResult<T> = Result<T, RepositoryError>;
