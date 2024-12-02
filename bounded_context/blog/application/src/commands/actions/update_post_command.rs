use shared_kernel::application::commands::ICommand;

#[derive(Debug, Clone)]
pub struct UpdatePostCommand {
    pub post_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl UpdatePostCommand {
    pub fn new(post_id: &str) -> Self {
        Self {
            post_id: post_id.to_string(),
            title: None,
            content: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }
}

impl ICommand for UpdatePostCommand {}
