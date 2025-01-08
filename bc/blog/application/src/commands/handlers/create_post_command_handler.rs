use async_trait::async_trait;
use blog_domain::{
    aggregate_root::{Post, PostAggregate},
    events::PostAggregateEvent,
    value_objects::post_id::PostId,
};
use shared_kernel::{
    application::{commands::ICommandHandler, events::EventDispatcher},
    domain::{date::IDateProvider, domain_event::EventStore},
};

use crate::commands::actions::CreatePostCommand;

pub struct CreatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent>,
    D: IDateProvider,
{
    event_dispatcher: EventDispatcher,
    date_provider: D,
    store: E,
}

impl<E, D> CreatePostCommandHandler<E, D>
where
    D: IDateProvider + Clone,
    E: EventStore<PostAggregateEvent> + Clone,
{
    pub fn new(store: &E, date_provider: &D, event_dispatcher: EventDispatcher) -> Self {
        Self {
            store: store.clone(),
            date_provider: date_provider.clone(),
            event_dispatcher,
        }
    }
}

#[async_trait]
impl<E, D> ICommandHandler<CreatePostCommand, PostId> for CreatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent>,
    D: IDateProvider,
{
    async fn execute(&self, command: &CreatePostCommand) -> anyhow::Result<PostId> {
        let occurred_on = self.date_provider.now();
        let post = Post::new(&command.title, &command.content);
        let mut post_aggregate = PostAggregate::new(post.clone(), &occurred_on);

        for event in post_aggregate.get_uncommitted_events() {
            self.store.append(event).await?;
        }

        for event in post_aggregate.get_uncommitted_events() {
            self.event_dispatcher.dispatch(event).await;
        }

        post_aggregate.clear_uncommitted_events();

        Ok(post.id)
    }
}
