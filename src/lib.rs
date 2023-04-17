/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use godot::prelude::*;
use prelude::CallableStatic;

mod bind_inner;
pub mod callable_static;
pub mod closures;
pub mod signal_inner;

pub trait IntoCallable<Args, R> {
    fn into_callable(self) -> Callable;

    fn into_static<const ARGC: usize>(self) -> CallableStatic<ARGC>
    where
        Self: Sized,
    {
        CallableStatic(self.into_callable())
    }
}

impl<T: GodotClass> IntoCallable<(), ()> for (Gd<T>, &'static str) {
    fn into_callable(self) -> Callable {
        Callable::from_object_method(self.0, self.1)
    }
}

impl IntoCallable<(), ()> for Callable {
    fn into_callable(self) -> Callable {
        self
    }
}

pub mod bind {
    pub use super::bind_inner::Bind as _;
}

pub mod signal {
    pub use super::signal_inner::{ConnectFlags, SignalProperty, SignalTrait as _};
}

pub mod prelude {
    pub use super::bind::*;
    pub use super::callable_static::*;
    pub use super::closures::*;
    pub use super::signal::*;
    pub use super::IntoCallable;
}
