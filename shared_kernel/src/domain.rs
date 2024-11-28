use std::fmt::Debug;

pub trait IAggregateRoot: IEntity {}

pub trait IEntity {}

pub trait IValueObject {}

pub trait IDomainEvent: Debug {}
