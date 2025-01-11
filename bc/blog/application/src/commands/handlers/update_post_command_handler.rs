use crate::commands::actions::UpdatePostCommand;
use async_trait::async_trait;
use blog_domain::aggregate_root::PostAggregateFactory;
use blog_domain::events::PostAggregateEvent;
use blog_domain::value_objects::post_id::PostId;
use blog_domain::PostSnapshot;
use shared_kernel::application::commands::ICommandHandler;
use shared_kernel::application::events::EventDispatcher;
use shared_kernel::domain::date::IDateProvider;
use shared_kernel::domain::domain_event::{EventStore, IDomainEvent, IObservableAggregateRoot};
use shared_kernel::domain::snapshot::SnapshotRepository;

pub struct UpdatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent> + SnapshotRepository<PostSnapshot>,
    D: IDateProvider,
{
    event_dispatcher: EventDispatcher,
    date_provider: D,
    event_store: E,
}

impl<E, D> UpdatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent> + SnapshotRepository<PostSnapshot> + Clone,
    D: IDateProvider + Clone,
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
impl<E, D> ICommandHandler<UpdatePostCommand, ()> for UpdatePostCommandHandler<E, D>
where
    E: EventStore<PostAggregateEvent> + SnapshotRepository<PostSnapshot>,
    D: IDateProvider,
{
    async fn execute(&self, command: &UpdatePostCommand) -> anyhow::Result<()> {
        let occurred_on = self.date_provider.now();
        let post_id = PostId::new(&command.post_id);
        let maybe_snapshot = self.event_store.load_snapshot(&command.post_id).await?;

        let (mut post_aggregate, post_aggregate_events) = if let Some(snapshot) = maybe_snapshot {
            let events = self
                .event_store
                .load_events_after_version(&command.post_id, snapshot.version)
                .await?;
            let post_aggregate = PostAggregateFactory::from_snapshot(snapshot);

            (post_aggregate, events)
        } else {
            let post_aggregate = PostAggregateFactory::from_id(post_id);
            let store = self
                .event_store
                .get_events_for_aggregate(&command.post_id)
                .await?;

            (post_aggregate, store)
        };

        let last_processed_version = post_aggregate_events.last().map_or(0, |e| e.version());

        for event in post_aggregate_events {
            post_aggregate.apply(event);
        }

        if let Some(title) = &command.title {
            post_aggregate.change_title(title, &occurred_on);
        }

        if let Some(content) = &command.content {
            post_aggregate.edit_content(content, &occurred_on);
        }

        for event in post_aggregate.get_uncommitted_events() {
            self.event_store.append(event).await?;
        }

        self.event_store
            .take_snapshot(post_aggregate.to_snapshot())
            .await?;

        for event in post_aggregate.get_uncommitted_events() {
            self.event_dispatcher
                .dispatch(event, last_processed_version)
                .await;
        }

        post_aggregate.clear_uncommitted_events();

        Ok(())
    }
}
