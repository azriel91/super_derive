use std::marker::PhantomData;

use super_derive::Super;

#[derive(Super)]
pub struct Man<T> {
    #[super_derive(skip)]
    name: String,
    power_level: u64,
    marker: PhantomData<T>,
}
