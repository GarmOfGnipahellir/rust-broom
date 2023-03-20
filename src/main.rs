use bevy::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(update.after(setup))
        .run();
}

struct Dioxus {
    pub dom: VirtualDom,
}

fn ui_root(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello world" }
    })
}

fn setup(world: &mut World) {
    let dom = VirtualDom::new(ui_root);
    world.insert_non_send_resource(Dioxus { dom });
}

fn update(mut dioxus: NonSendMut<Dioxus>) {
    let mutations = dioxus.dom.rebuild();

    dbg!(mutations.templates);
    dbg!(mutations.edits);
}
