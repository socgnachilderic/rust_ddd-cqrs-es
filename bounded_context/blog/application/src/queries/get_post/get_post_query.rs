use shared_kernel::application::IQuery;

#[derive(Debug, Clone)]
pub struct GetPostQuery(pub String);

impl GetPostQuery {
    pub fn new(post_id: &str) -> Self {
        GetPostQuery(post_id.to_string())
    }
}

impl IQuery for GetPostQuery {}