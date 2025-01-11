use shared_kernel::domain::date::Date;
use shared_kernel::domain::domain_event::{IApplyDomainEvent, IObservableAggregateRoot};
use shared_kernel::domain::{IAggregateRoot, IEntity};

use crate::entities::comment::Comment;
use crate::events::{
    PostAggregateEvent, PostContentEditedEvent, PostCreatedEvent, PostTitleChangedEvent,
};
use crate::r#enum::post_state::PostState;
use crate::value_objects::comment_id::CommentId;
use crate::value_objects::post_id::PostId;
use crate::PostSnapshot;

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
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: PostId::generate(),
            title: Default::default(),
            content: Default::default(),
            state: Default::default(),
            comments: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostAggregate {
    pub post: Post,
    pub version: i64,
    pub recorded_events: Vec<PostAggregateEvent>,
}

impl PostAggregate {
    pub fn new(title: &str, content: &str, occurred_on: &Date) -> Self {
        let post_id = PostId::generate();
        let mut aggregate = PostAggregateFactory::from_id(post_id.clone());

        let event = PostCreatedEvent {
            post_id,
            title: title.to_string(),
            content: content.to_string(),
            occurred_on: occurred_on.clone(),
            version: 1,
        };

        aggregate.apply(event.into());

        aggregate
    }

    pub fn change_title(&mut self, title: &str, occurred_on: &Date) {
        let event = PostTitleChangedEvent {
            post_id: self.post.id.clone(),
            title: title.to_string(),
            occurred_on: occurred_on.clone(),
            version: self.version,
        };

        self.apply(event.into());
    }

    pub fn edit_content(&mut self, content: &str, occurred_on: &Date) {
        let event = PostContentEditedEvent {
            post_id: self.post.id.clone(),
            content: content.to_string(),
            occurred_on: occurred_on.clone(),
            version: self.version,
        };

        self.apply(event.into());
    }
}

impl IObservableAggregateRoot<PostAggregate> for PostAggregate {
    type Event = PostAggregateEvent;

    fn get_uncommitted_events(&mut self) -> &[PostAggregateEvent] {
        &self.recorded_events
    }

    fn clear_uncommitted_events(&mut self) {
        self.recorded_events.clear();
    }

    fn apply(&mut self, event: PostAggregateEvent) {
        event.apply_to(self);
        self.recorded_events.push(event);
        self.version += 1;
    }
}

impl PostAggregate {
    pub fn to_snapshot(&self) -> PostSnapshot {
        PostSnapshot {
            id: self.post.id.to_string(),
            title: self.post.title.clone(),
            content: self.post.content.clone(),
            version: self.version,
        }
    }
}

impl IEntity for PostAggregate {}

impl IAggregateRoot for PostAggregate {}

impl PartialEq for PostAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.post == other.post
    }
}

pub struct PostAggregateFactory;

impl PostAggregateFactory {
    pub fn from_snapshot(snapshot: PostSnapshot) -> PostAggregate {
        PostAggregate {
            post: Post::new_with_id(snapshot.id.into(), &snapshot.title, &snapshot.content),
            recorded_events: vec![],
            version: snapshot.version,
        }
    }

    pub fn from_id(post_id: PostId) -> PostAggregate {
        PostAggregate {
            post: Post::new_with_id(post_id, "", ""),
            recorded_events: vec![],
            version: 0,
        }
    }
}
