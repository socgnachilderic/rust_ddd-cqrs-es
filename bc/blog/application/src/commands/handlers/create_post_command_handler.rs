use async_trait::async_trait;
use blog_domain::PostSnapshot;
use blog_domain::{
    aggregate_root::PostAggregate, events::PostAggregateEvent, value_objects::post_id::PostId,
};
use shared_kernel::application::{commands::ICommandHandler, events::EventDispatcher};
use shared_kernel::domain::date::IDateProvider;
use shared_kernel::domain::domain_event::{EventStore, IObservableAggregateRoot};
use shared_kernel::domain::snapshot::SnapshotRepository;

use crate::commands::actions::CreatePostCommand;

pub struct CreatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent> + SnapshotRepository<PostSnapshot>,
    D: IDateProvider,
{
    event_dispatcher: EventDispatcher,
    date_provider: D,
    event_store: E,
}

impl<E, D> CreatePostCommandHandler<E, D>
where
    D: IDateProvider + Clone,
    E: EventStore<PostAggregateEvent> + SnapshotRepository<PostSnapshot> + Clone,
{
    pub fn new(store: &E, date_provider: &D, event_dispatcher: EventDispatcher) -> Self {
        Self {
            event_store: store.clone(),
            date_provider: date_provider.clone(),
            event_dispatcher,
        }
    }
}

#[async_trait]
impl<E, D> ICommandHandler<CreatePostCommand, PostId> for CreatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent> + SnapshotRepository<PostSnapshot>,
    D: IDateProvider,
{
    async fn execute(&self, command: &CreatePostCommand) -> anyhow::Result<PostId> {
        let occurred_on = self.date_provider.now();
        let mut post_aggregate = PostAggregate::new(&command.title, &command.content, &occurred_on);

        for event in post_aggregate.get_uncommitted_events() {
            self.event_store.append(event).await?;
        }

        self.event_store
            .take_snapshot(post_aggregate.to_snapshot())
            .await?;

        for event in post_aggregate.get_uncommitted_events() {
            self.event_dispatcher.dispatch(event, 0).await;
        }

        post_aggregate.clear_uncommitted_events();

        Ok(post_aggregate.post.id)
    }
}
