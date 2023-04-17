/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use godot::prelude::*;

use crate::prelude::CallableStatic;
use crate::IntoCallable;

pub trait Bind<T>
where
    T: ToVariant + 'static,
{
    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 1 argument.
    fn bind1(self, arg: T) -> CallableStatic<0>
    where
        Self: Into<CallableStatic<1>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 2 arguments.
    fn bind2(self, arg: T) -> CallableStatic<1>
    where
        Self: Into<CallableStatic<2>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 3 arguments.
    fn bind3(self, arg: T) -> CallableStatic<2>
    where
        Self: Into<CallableStatic<3>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 4 arguments.
    fn bind4(self, arg: T) -> CallableStatic<3>
    where
        Self: Into<CallableStatic<4>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 5 arguments.
    fn bind5(self, arg: T) -> CallableStatic<4>
    where
        Self: Into<CallableStatic<5>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 6 arguments.
    fn bind6(self, arg: T) -> CallableStatic<5>
    where
        Self: Into<CallableStatic<6>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 7 arguments.
    fn bind7(self, arg: T) -> CallableStatic<6>
    where
        Self: Into<CallableStatic<7>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 8 arguments.
    fn bind8(self, arg: T) -> CallableStatic<7>
    where
        Self: Into<CallableStatic<8>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 9 arguments.
    fn bind9(self, arg: T) -> CallableStatic<8>
    where
        Self: Into<CallableStatic<9>>;

    /// Return a copy of this callable with the last argument bound to the given argument.
    ///
    /// Assuming the callable already takes 10 arguments.
    fn bind10(self, arg: T) -> CallableStatic<9>
    where
        Self: Into<CallableStatic<10>>;
}

impl<T> Bind<T> for Callable
where
    T: ToVariant + 'static,
{
    fn bind1(self, arg: T) -> CallableStatic<0> {
        (move || self.callv(varray![arg.to_variant()]))
            .into_callable()
            .into()
    }

    fn bind2(self, arg: T) -> CallableStatic<1>
    where
        Self: Into<CallableStatic<2>>,
    {
        (move |arg1: Variant| self.callv(varray![arg1, arg.to_variant()]))
            .into_callable()
            .into()
    }

    fn bind3(self, arg: T) -> CallableStatic<2>
    where
        Self: Into<CallableStatic<3>>,
    {
        (move |arg1: Variant, arg2: Variant| self.callv(varray![arg1, arg2, arg.to_variant()]))
            .into_callable()
            .into()
    }

    fn bind4(self, arg: T) -> CallableStatic<3>
    where
        Self: Into<CallableStatic<4>>,
    {
        (move |arg1: Variant, arg2: Variant, arg3: Variant| {
            self.callv(varray![arg1, arg2, arg3, arg.to_variant()])
        })
        .into_callable()
        .into()
    }

    fn bind5(self, arg: T) -> CallableStatic<4>
    where
        Self: Into<CallableStatic<5>>,
    {
        (move |arg1: Variant, arg2: Variant, arg3: Variant, arg4: Variant| {
            self.callv(varray![arg1, arg2, arg3, arg4, arg.to_variant()])
        })
        .into_callable()
        .into()
    }

    fn bind6(self, arg: T) -> CallableStatic<5>
    where
        Self: Into<CallableStatic<6>>,
    {
        (move |arg1: Variant, arg2: Variant, arg3: Variant, arg4: Variant, arg5: Variant| {
            self.callv(varray![arg1, arg2, arg3, arg4, arg5, arg.to_variant()])
        })
        .into_callable()
        .into()
    }

    fn bind7(self, arg: T) -> CallableStatic<6>
    where
        Self: Into<CallableStatic<7>>,
    {
        (move |arg1: Variant,
               arg2: Variant,
               arg3: Variant,
               arg4: Variant,
               arg5: Variant,
               arg6: Variant| {
            self.callv(varray![
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
                arg.to_variant()
            ])
        })
        .into_callable()
        .into()
    }

    fn bind8(self, arg: T) -> CallableStatic<7>
    where
        Self: Into<CallableStatic<8>>,
    {
        (move |arg1: Variant,
               arg2: Variant,
               arg3: Variant,
               arg4: Variant,
               arg5: Variant,
               arg6: Variant,
               arg7: Variant| {
            self.callv(varray![
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
                arg7,
                arg.to_variant()
            ])
        })
        .into_callable()
        .into()
    }

    fn bind9(self, arg: T) -> CallableStatic<8>
    where
        Self: Into<CallableStatic<9>>,
    {
        (move |arg1: Variant,
               arg2: Variant,
               arg3: Variant,
               arg4: Variant,
               arg5: Variant,
               arg6: Variant,
               arg7: Variant,
               arg8: Variant| {
            self.callv(varray![
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
                arg7,
                arg8,
                arg.to_variant()
            ])
        })
        .into_callable()
        .into()
    }

    fn bind10(self, arg: T) -> CallableStatic<9>
    where
        Self: Into<CallableStatic<10>>,
    {
        (move |arg1: Variant,
               arg2: Variant,
               arg3: Variant,
               arg4: Variant,
               arg5: Variant,
               arg6: Variant,
               arg7: Variant,
               arg8: Variant,
               arg9: Variant| {
            self.callv(varray![
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
                arg7,
                arg8,
                arg9,
                arg.to_variant()
            ])
        })
        .into_callable()
        .into()
    }
}

impl<T, const ARGC: usize> Bind<T> for CallableStatic<ARGC>
where
    T: ToVariant + 'static,
{
    fn bind1(self, arg: T) -> CallableStatic<0>
    where
        Self: Into<CallableStatic<1>>,
    {
        self.0.bind1(arg)
    }

    fn bind2(self, arg: T) -> CallableStatic<1>
    where
        Self: Into<CallableStatic<2>>,
    {
        self.0.bind2(arg)
    }

    fn bind3(self, arg: T) -> CallableStatic<2>
    where
        Self: Into<CallableStatic<3>>,
    {
        self.0.bind3(arg)
    }

    fn bind4(self, arg: T) -> CallableStatic<3>
    where
        Self: Into<CallableStatic<4>>,
    {
        self.0.bind4(arg)
    }

    fn bind5(self, arg: T) -> CallableStatic<4>
    where
        Self: Into<CallableStatic<5>>,
    {
        self.0.bind5(arg)
    }

    fn bind6(self, arg: T) -> CallableStatic<5>
    where
        Self: Into<CallableStatic<6>>,
    {
        self.0.bind6(arg)
    }

    fn bind7(self, arg: T) -> CallableStatic<6>
    where
        Self: Into<CallableStatic<7>>,
    {
        self.0.bind7(arg)
    }

    fn bind8(self, arg: T) -> CallableStatic<7>
    where
        Self: Into<CallableStatic<8>>,
    {
        self.0.bind8(arg)
    }

    fn bind9(self, arg: T) -> CallableStatic<8>
    where
        Self: Into<CallableStatic<9>>,
    {
        self.0.bind9(arg)
    }

    fn bind10(self, arg: T) -> CallableStatic<9>
    where
        Self: Into<CallableStatic<10>>,
    {
        self.0.bind10(arg)
    }
}
