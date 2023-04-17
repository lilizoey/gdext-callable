/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use godot::prelude::*;

use crate::{bind_inner::Bind, IntoCallable};

#[derive(Debug, Clone, PartialEq)]
/// A callable that takes a known number of arguments.
pub struct CallableStatic<const ARGC: usize>(pub Callable);

impl CallableStatic<0> {
    /// Call the underlying callable with 0 arguments.
    pub fn call(&self) -> Variant {
        self.0.callv(varray![])
    }
}

impl CallableStatic<1> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<0> {
        self.bind1(arg)
    }

    /// Call the underlying callable with 1 argument.
    pub fn call<Arg1: ToVariant>(&self, arg1: Arg1) -> Variant {
        self.0.callv(varray![arg1.to_variant()])
    }
}

impl CallableStatic<2> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<1> {
        self.bind2(arg)
    }

    /// Call the underlying callable with 2 arguments.
    pub fn call<Arg1: ToVariant, Arg2: ToVariant>(&self, arg1: Arg1, arg2: Arg2) -> Variant {
        self.0.callv(varray![arg1.to_variant(), arg2.to_variant()])
    }
}

impl CallableStatic<3> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<2> {
        self.bind3(arg)
    }

    /// Call the underlying callable with 3 arguments.
    pub fn call<Arg1: ToVariant, Arg2: ToVariant, Arg3: ToVariant>(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant()
        ])
    }
}

impl CallableStatic<4> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<3> {
        self.bind4(arg)
    }

    /// Call the underlying callable with 4 arguments.
    pub fn call<Arg1: ToVariant, Arg2: ToVariant, Arg3: ToVariant, Arg4: ToVariant>(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant()
        ])
    }
}

impl CallableStatic<5> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<4> {
        self.bind5(arg)
    }

    /// Call the underlying callable with 5 arguments.
    pub fn call<
        Arg1: ToVariant,
        Arg2: ToVariant,
        Arg3: ToVariant,
        Arg4: ToVariant,
        Arg5: ToVariant,
    >(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
        arg5: Arg5,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant(),
            arg5.to_variant()
        ])
    }
}

impl CallableStatic<6> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<5> {
        self.bind6(arg)
    }

    /// Call the underlying callable with 6 arguments.
    pub fn call<
        Arg1: ToVariant,
        Arg2: ToVariant,
        Arg3: ToVariant,
        Arg4: ToVariant,
        Arg5: ToVariant,
        Arg6: ToVariant,
    >(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
        arg5: Arg5,
        arg6: Arg6,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant(),
            arg5.to_variant(),
            arg6.to_variant()
        ])
    }
}

impl CallableStatic<7> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<6> {
        self.bind7(arg)
    }

    /// Call the underlying callable with 7 arguments.
    pub fn call<
        Arg1: ToVariant,
        Arg2: ToVariant,
        Arg3: ToVariant,
        Arg4: ToVariant,
        Arg5: ToVariant,
        Arg6: ToVariant,
        Arg7: ToVariant,
    >(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
        arg5: Arg5,
        arg6: Arg6,
        arg7: Arg7,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant(),
            arg5.to_variant(),
            arg6.to_variant(),
            arg7.to_variant()
        ])
    }
}

impl CallableStatic<8> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<7> {
        self.bind8(arg)
    }

    /// Call the underlying callable with 8 arguments.
    pub fn call<
        Arg1: ToVariant,
        Arg2: ToVariant,
        Arg3: ToVariant,
        Arg4: ToVariant,
        Arg5: ToVariant,
        Arg6: ToVariant,
        Arg7: ToVariant,
        Arg8: ToVariant,
    >(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
        arg5: Arg5,
        arg6: Arg6,
        arg7: Arg7,
        arg8: Arg8,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant(),
            arg5.to_variant(),
            arg6.to_variant(),
            arg7.to_variant(),
            arg8.to_variant()
        ])
    }
}

impl CallableStatic<9> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<8> {
        self.bind9(arg)
    }

    /// Call the underlying callable with 9 arguments.
    pub fn call<
        Arg1: ToVariant,
        Arg2: ToVariant,
        Arg3: ToVariant,
        Arg4: ToVariant,
        Arg5: ToVariant,
        Arg6: ToVariant,
        Arg7: ToVariant,
        Arg8: ToVariant,
        Arg9: ToVariant,
    >(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
        arg5: Arg5,
        arg6: Arg6,
        arg7: Arg7,
        arg8: Arg8,
        arg9: Arg9,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant(),
            arg5.to_variant(),
            arg6.to_variant(),
            arg7.to_variant(),
            arg8.to_variant(),
            arg9.to_variant()
        ])
    }
}

impl CallableStatic<10> {
    /// Return a copy of this callable with the last argument bound to the given argument.
    pub fn bind<T: ToVariant + 'static>(self, arg: T) -> CallableStatic<9> {
        self.bind10(arg)
    }

    /// Call the underlying callable with 10 arguments.
    pub fn call<
        Arg1: ToVariant,
        Arg2: ToVariant,
        Arg3: ToVariant,
        Arg4: ToVariant,
        Arg5: ToVariant,
        Arg6: ToVariant,
        Arg7: ToVariant,
        Arg8: ToVariant,
        Arg9: ToVariant,
        Arg10: ToVariant,
    >(
        &self,
        arg1: Arg1,
        arg2: Arg2,
        arg3: Arg3,
        arg4: Arg4,
        arg5: Arg5,
        arg6: Arg6,
        arg7: Arg7,
        arg8: Arg8,
        arg9: Arg9,
        arg10: Arg10,
    ) -> Variant {
        self.0.callv(varray![
            arg1.to_variant(),
            arg2.to_variant(),
            arg3.to_variant(),
            arg4.to_variant(),
            arg5.to_variant(),
            arg6.to_variant(),
            arg7.to_variant(),
            arg8.to_variant(),
            arg9.to_variant(),
            arg10.to_variant()
        ])
    }
}

impl<const ARGC: usize> IntoCallable<(), ()> for CallableStatic<ARGC> {
    fn into_callable(self) -> Callable {
        self.0
    }
}

impl<const ARGC: usize> From<Callable> for CallableStatic<ARGC> {
    fn from(callable: Callable) -> Self {
        CallableStatic(callable)
    }
}
