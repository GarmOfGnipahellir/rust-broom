use bevy::prelude::{App, Plugin};
use dioxus::prelude::*;

pub mod dioxus_elements {
    pub struct node;
    impl node {
        pub const TAG_NAME: &'static str = "node";
        pub const NAME_SPACE: Option<&'static str> = None;

        pub const background_color: (&'static str, Option<&'static str>, bool) =
            ("background_color", None, false);
    }
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        node {
            background_color: "#ff0000",
            "hello world"
        }
    })
}

fn startup() {
    let mut vdom = VirtualDom::new(app);
    let mutations = vdom.rebuild();
    dbg!(&mutations);
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}
