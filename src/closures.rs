/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use super::IntoCallable;
use godot::prelude::*;

macro_rules! impl_into_callable {
    (@struct_def $struct_name:ident, $func:ty) => {
        #[derive(GodotClass)]
        #[class(base=RefCounted)]
        pub struct $struct_name {
            func: Box<$func>,
        }
    };
    (@struct_impl $struct_name:ident, $($arg:ident: $variant:ident),*) => {
        #[godot_api]
        impl $struct_name {
            #[func]
            fn __call_closure(&self, $($arg: $variant),*) -> Variant {
                (self.func)($($arg),*)
            }
        }
    };
    (@into_callable $struct_name:ident, $args:tt, ($($arg_generic:ident),*), ($($arg:ident),*)) => {
        impl<F, R, $($arg_generic),*> IntoCallable<$args, R> for F
        where
            F: Fn($($arg_generic),*) -> R + 'static,
            R: ToVariant + 'static,
            $($arg_generic: FromVariant + 'static),*
        {
            fn into_callable(self) -> Callable {
                let func = move |$($arg: Variant),*| {
                    self($($arg.to()),*).to_variant()
                };
                let obj = Gd::<$struct_name>::new($struct_name {
                    func: Box::new(func)
                });
                let callable = Callable::from_object_method(obj.share(), "__call_closure");
                std::mem::forget(obj);
                callable
            }
        }
    };
    (@make_var_func $($arg:ident: $variant:ident),*) => {
        dyn Fn($($variant),*) -> Variant
    };
    ($struct_name:ident) => {
        impl_into_callable!(@struct_def $struct_name, dyn Fn() -> Variant);
        impl_into_callable!(@struct_impl $struct_name,);
        impl_into_callable!(@into_callable $struct_name, (), (), ());
    };
    ($struct_name:ident, ($arg:ident, $generic:ident)) => {
        impl_into_callable!(@struct_def $struct_name, dyn Fn(Variant) -> Variant);
        impl_into_callable!(@struct_impl $struct_name, $arg: Variant);
        impl_into_callable!(@into_callable $struct_name, ($generic,), ($generic), ($arg));
    };
    ($struct_name:ident, ($arg:ident, $generic:ident), $(($args:ident, $generics:ident)),+) => {
        impl_into_callable!(
            @struct_def $struct_name,
            impl_into_callable!(@make_var_func $arg: Variant, $($args: Variant),+)
        );
        impl_into_callable!(
            @struct_impl $struct_name,
            $arg: Variant, $($args: Variant),+
        );
        impl_into_callable!(
            @into_callable $struct_name,
            ($generic, $($generics),+),
            ($generic, $($generics),+),
            ($arg, $($args),+)
        );
    };
}

macro_rules! impl_into_callables {
    () => {
        impl_into_callable!(CallableClosure);
    };
    (($struct_name:ident), ($arg:ident, $generic:ident) $(,)?) => {
        impl_into_callable!($struct_name, ($arg, $generic));
        impl_into_callables!();
    };
    (($struct_name:ident, $($struct_names:ident),+), ($arg:ident, $generic:ident), $(($args:ident, $generics:ident)),+ $(,)?) => {
        impl_into_callables!(@inner $struct_name, $(($args, $generics)),+; ($arg, $generic));
        impl_into_callables!(($($struct_names),+), $(($args, $generics)),+);
    };
    (@inner $struct_name:ident, ($arg:ident, $generic:ident), $(($args:ident, $generics:ident)),+; $($t:tt)*) => {
        impl_into_callables!(@inner $struct_name, $(($args, $generics)),+; ($arg, $generic), $($t)*);
    };
    (@inner $struct_name:ident, ($arg:ident, $generic:ident); $($t:tt)*) => {
        impl_into_callable!($struct_name, ($arg, $generic), $($t)*);
    };
}

impl_into_callables!(
    (
        CallableClosure10,
        CallableClosure9,
        CallableClosure8,
        CallableClosure7,
        CallableClosure6,
        CallableClosure5,
        CallableClosure4,
        CallableClosure3,
        CallableClosure2,
        CallableClosure1
    ),
    (arg10, Arg10),
    (arg9, Arg9),
    (arg8, Arg8),
    (arg7, Arg7),
    (arg6, Arg6),
    (arg5, Arg5),
    (arg4, Arg4),
    (arg3, Arg3),
    (arg2, Arg2),
    (arg1, Arg1),
);
