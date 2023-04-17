/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use super::IntoCallable;
use godot::{engine::global, prelude::*};

/// Options for modifying how a connection happens.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectFlags {
    /// Deferred connections trigger their Callables on idle time, rather than instantly.
    deferred: bool,
    /// Persisting connections are stored when the object is serialized (such as when using
    /// PackedScene.pack). In the editor, connections created through the Node dock are always persisting.
    persist: bool,
    /// One-shot connections disconnect themselves after emission.
    one_shot: bool,
    /// Reference-counted connections can be assigned to the same Callable multiple times. Each disconnection
    /// decreases the internal counter. The signal fully disconnects only when the counter reaches 0.
    ref_counted: bool,
}

impl From<ConnectFlags> for i64 {
    fn from(flags: ConnectFlags) -> Self {
        let ConnectFlags {
            deferred,
            persist,
            one_shot,
            ref_counted,
        } = flags;

        deferred as i64 | (persist as i64) << 1 | (one_shot as i64) << 2 | (ref_counted as i64) << 3
    }
}

impl ConnectFlags {
    pub const DEFERRED: Self = Self {
        deferred: true,
        persist: false,
        one_shot: false,
        ref_counted: false,
    };
    pub const PERSIST: Self = Self {
        deferred: false,
        persist: true,
        one_shot: false,
        ref_counted: false,
    };
    pub const ONE_SHOT: Self = Self {
        deferred: false,
        persist: false,
        one_shot: true,
        ref_counted: false,
    };
    pub const REF_COUNTED: Self = Self {
        deferred: false,
        persist: false,
        one_shot: false,
        ref_counted: true,
    };

    /// Create a new connectflags with the given settings.
    pub const fn new(deferred: bool, persist: bool, one_shot: bool, ref_counted: bool) -> Self {
        Self {
            deferred,
            persist,
            one_shot,
            ref_counted,
        }
    }

    /// Combine two connect flags so both the connections happen.
    pub const fn and(self, other: Self) -> Self {
        Self {
            deferred: self.deferred || other.deferred,
            persist: self.persist || other.persist,
            one_shot: self.one_shot || other.one_shot,
            ref_counted: self.ref_counted || other.ref_counted,
        }
    }
}

/// The name and type of an argument that a signal takes.
pub struct SignalProperty {
    name: GodotString,
    type_: VariantType,
}

impl ToVariant for SignalProperty {
    fn to_variant(&self) -> Variant {
        dict! {
            "name": self.name.clone(),
            "type": self.type_ as i64
        }
        .to_variant()
    }
}

impl<S: Into<GodotString>> From<(S, VariantType)> for SignalProperty {
    fn from((name, type_): (S, VariantType)) -> Self {
        Self {
            name: name.into(),
            type_,
        }
    }
}

macro_rules! gen_constructors {
    ($($name:ident),* $(,)?) => {
        $(
            #[doc = concat!(" Create a new signal property of type `", stringify!($name), "`")]
            pub fn $name(name: impl Into<GodotString>) -> Self {
                Self {
                    name: name.into(),
                    type_: VariantType::$name
                }
            }
        )*
    };
}

#[allow(non_snake_case)]
impl SignalProperty {
    gen_constructors!(
        Nil,
        Bool,
        Int,
        Float,
        String,
        Vector2,
        Vector2i,
        Rect2,
        Rect2i,
        Vector3,
        Vector3i,
        Transform2D,
        Vector4,
        Vector4i,
        Plane,
        Quaternion,
        Aabb,
        Basis,
        Transform3D,
        Projection,
        Color,
        StringName,
        NodePath,
        Rid,
        Object,
        Callable,
        Signal,
        Dictionary,
        Array,
        PackedByteArray,
        PackedInt32Array,
        PackedInt64Array,
        PackedFloat32Array,
        PackedFloat64Array,
        PackedStringArray,
        PackedVector2Array,
        PackedVector3Array,
        PackedColorArray,
    );
}

pub trait SignalTrait<T> {
    /// Connect a signal with custom flags.
    fn connect_flags<C, Args, R>(
        &self,
        signal_name: impl Into<StringName>,
        callable: C,
        flags: ConnectFlags,
    ) -> Result<(), global::Error>
    where
        C: IntoCallable<Args, R>;

    /// Connect a signal with no flags set.
    fn connect_default<C, Args, R>(
        &self,
        signal_name: impl Into<StringName>,
        callable: C,
    ) -> Result<(), global::Error>
    where
        C: IntoCallable<Args, R>,
    {
        self.connect_flags(signal_name, callable, ConnectFlags::default())
    }

    /// Add a signal with the given properties.
    fn add_signal<S: Into<SignalProperty>, I: IntoIterator<Item = S>>(
        &self,
        name: impl Into<GodotString>,
        properties: I,
    );

    /// Add a signal that takes no properties.
    fn add_signal_none(&self, name: impl Into<GodotString>) {
        self.add_signal::<SignalProperty, [SignalProperty; 0]>(name, [])
    }
}

impl<T: GodotClass + Inherits<Object>> SignalTrait<T> for Gd<T> {
    fn connect_flags<C, Args, R>(
        &self,
        signal_name: impl Into<StringName>,
        callable: C,
        flags: ConnectFlags,
    ) -> Result<(), global::Error>
    where
        C: IntoCallable<Args, R>,
    {
        let result = self.share().upcast::<Object>().connect(
            signal_name.into(),
            callable.into_callable(),
            flags.into(),
        );
        if result == global::Error::OK {
            Ok(())
        } else {
            Err(result)
        }
    }

    fn add_signal<S: Into<SignalProperty>, I: IntoIterator<Item = S>>(
        &self,
        name: impl Into<GodotString>,
        properties: I,
    ) {
        self.share().upcast::<Object>().add_user_signal(
            name.into(),
            properties
                .into_iter()
                .map(|s| s.into().to_variant())
                .collect(),
        )
    }
}
