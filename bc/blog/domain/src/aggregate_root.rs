use shared_kernel::domain::date::Date;

use crate::entities::comment::Comment;
use crate::events::{PostAggregateEvent, PostCreatedEvent};
use crate::r#enum::post_state::PostState;
use crate::value_objects::comment_id::CommentId;
use crate::value_objects::post_id::PostId;

#[derive(Debug, Clone)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
    pub state: PostState,
    pub comments: Vec<Comment>,
}

impl Post {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            id: PostId::generate(),
            comments: vec![],
            state: PostState::StateDraft,
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    pub fn new_with_id(id: PostId, title: &str, content: &str) -> Self {
        Self {
            id,
            comments: vec![],
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
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct PostAggregate {
    pub post: Post,
    pub version: i64,
    pub recorded_events: Vec<PostAggregateEvent>,
}

impl PostAggregate {
    pub fn new(post: Post, occurred_on: &Date) -> Self {
        let mut aggregate = Self {
            post: post.clone(),
            recorded_events: vec![],
            version: 0,
        };

        let event = PostCreatedEvent {
            post_id: post.id,
            title: post.title,
            content: post.content,
            occurred_on: occurred_on.clone(),
            version: aggregate.version,
        };

        aggregate.apply(event.into());

        aggregate
    }

    pub fn get_uncommitted_events(&mut self) -> &[PostAggregateEvent] {
        &self.recorded_events
    }

    pub fn clear_uncommitted_events(&mut self) {
        self.recorded_events.clear();
    }

    fn apply(&mut self, event: PostAggregateEvent) {
        self.recorded_events.push(event);
    }
}

impl PartialEq for PostAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.post == other.post
    }
}
