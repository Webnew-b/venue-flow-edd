use thiserror::Error;


#[derive(Debug,Error)]
pub enum DatabaseError {
    #[error("Other message:{0}")]
    Other(String)
}
