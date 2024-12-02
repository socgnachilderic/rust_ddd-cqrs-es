use shared_kernel::application::queries::IQuery;

#[derive(Debug, Clone)]
pub struct GetPostQuery(pub String);

impl From<String> for GetPostQuery {
    fn from(id: String) -> Self {
        GetPostQuery(id)
    }
}

impl IQuery for GetPostQuery {}
