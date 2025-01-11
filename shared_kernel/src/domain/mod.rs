pub mod date;
pub mod domain_event;
pub mod snapshot;

pub use domain_event::IDomainEvent;

pub trait IValueObject {}

pub trait IEntity {}

pub trait IAggregateRoot: IEntity {}
