## warning
This library leaks memory every time a closure is turned into a callable.

## usage
make dependencies look like:
```toml
[dependencies]
godot = { git = "https://github.com/lilizoey/gdextension", branch = "feature/callable" }
gdext-callable = { git = "https://github.com/lilizoey/gdext-callable", branch = "main" }
```

## example

```rs
use gdext_callable::prelude::*;
use godot::{engine::Engine, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Mini {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for Mini {
    fn ready(&mut self) {
        self.base.add_signal_none("my_signal_1");
        self.base
            .add_signal("my_signal_2", [SignalProperty::Int("prop_1")]);
        let callable = self.base.callable("foo").bind2(20);
        self.base.connect_default("my_signal_2", callable).unwrap();
        self.base
            .connect_default("my_signal_2", |prop_1: i32| godot_print!("got {prop_1}!"))
            .unwrap();
    }

    fn process(&mut self, _delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        if Input::singleton().is_action_just_pressed("ui_accept".into(), false) {
            // cannot emit signal directly as it causes aliasing issue
            self.base.call_deferred(
                "emit_signal".into(),
                &[
                    StringName::from("my_signal_2").to_variant(),
                    55i32.to_variant(),
                ],
            );
        }
    }
}

#[godot_api]
impl Mini {
    #[func]
    fn foo(&self, prop_1: i32, prop_2: i32) {
        godot_print!("first: {}, second: {}", prop_1, prop_2);
    }
}

struct Testing;

#[gdextension]
unsafe impl ExtensionLibrary for Testing {}
```
