use shared_kernel::application::queries::Query;

#[derive(Debug, Clone, Query)]
pub struct GetPostQuery(pub String);

impl From<String> for GetPostQuery {
    fn from(id: String) -> Self {
        GetPostQuery(id)
    }
}
