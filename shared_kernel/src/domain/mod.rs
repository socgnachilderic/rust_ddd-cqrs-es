pub mod date;
pub mod domain_event;

pub trait IAggregateRoot: IEntity {}

pub trait IEntity {}

pub trait IValueObject {}
