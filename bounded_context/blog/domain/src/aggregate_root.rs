use crate::entities::comment::Comment;
use crate::r#enum::post_state::PostState;
use crate::value_objects::comment_id::CommentId;
use crate::value_objects::post_id::PostId;
use shared_kernel::domain::IDomainEvent;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
    pub state: PostState,
    pub comments: Vec<Comment>,
    pub recorded_events: Vec<Arc<dyn IDomainEvent>>,
}

impl Post {
    pub fn new(id: PostId, title: &str, content: &str) -> Self {
        Self {
            id,
            comments: vec![],
            recorded_events: vec![],
            state: PostState::StateDraft,
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    pub fn publish(&mut self) {
        self.state = PostState::StatePublished
    }

    pub fn comment(&mut self, comment: &str) {
        self.comments
            .push(Comment::new(CommentId::generate(), comment));
    }

    pub fn change_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn edit_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    // fn create_empty_post_with(post_id: PostId) -> Self {
    //     Self::new(post_id, "", "")
    // }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
