use shared_kernel::application::queries::IQuery;

pub struct GetAllPostQuery;

impl IQuery for GetAllPostQuery {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
