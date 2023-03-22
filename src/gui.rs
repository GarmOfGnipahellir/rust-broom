use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use dioxus::{
    core::{ElementId, Mutation, Mutations},
    prelude::*,
};

pub mod dioxus_elements {
    pub struct node;
    impl node {
        pub const TAG_NAME: &'static str = "node";
        pub const NAME_SPACE: Option<&'static str> = None;

        pub const background_color: (&'static str, Option<&'static str>, bool) =
            ("background_color", None, false);
    }

    pub struct component;
    impl component {
        pub const TAG_NAME: &'static str = "component";
        pub const NAME_SPACE: Option<&'static str> = None;
    }

    pub struct bundle;
    impl bundle {
        pub const TAG_NAME: &'static str = "bundle";
        pub const NAME_SPACE: Option<&'static str> = None;

        pub const components: (&'static str, Option<&'static str>, bool) =
            ("components", None, false);
    }
}

#[derive(Resource, Debug)]
pub struct GuiContext {
    stack: Vec<Entity>,
    nodes: HashMap<ElementId, Entity>,
}

impl FromWorld for GuiContext {
    fn from_world(world: &mut World) -> Self {
        let root = world
            .spawn(NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        let mut nodes = HashMap::new();
        nodes.insert(ElementId(0), root);
        Self {
            stack: vec![],
            nodes,
        }
    }
}

#[derive(SystemParam)]
pub struct GuiParams<'w, 's> {
    pub ctx: ResMut<'w, GuiContext>,
    pub commands: Commands<'w, 's>,
}

impl<'w, 's> GuiParams<'w, 's> {
    pub fn spawn_template_node(&mut self, node: &TemplateNode) -> Entity {
        use dioxus::core::TemplateNode::*;
        match node {
            Element {
                tag,
                namespace,
                attrs,
                children,
            } => todo!(),
            Text { text } => todo!(),
            Dynamic { id } => todo!(),
            DynamicText { id } => todo!(),
        }
        self.commands.spawn(NodeBundle::default()).id()
    }

    pub fn apply_mutation(&mut self, templates: &HashMap<&str, Template>, mutation: &Mutation) {
        use dioxus::core::Mutation::*;
        match mutation {
            AppendChildren { id, m } => {
                let parent = *self.ctx.nodes.get(id).unwrap();
                for _ in 0..*m {
                    let child = self.ctx.stack.pop().unwrap();
                    self.commands.entity(parent).add_child(child);
                }
            }
            AssignId { path, id } => todo!(),
            CreatePlaceholder { id } => todo!(),
            CreateTextNode { value, id } => todo!(),
            HydrateText { path, value, id } => todo!(),
            LoadTemplate { name, index, id } => {
                let template = templates.get(name).unwrap();
                let root = template.roots.get(*index).unwrap();
                let entity = self.spawn_template_node(root);
                self.ctx.stack.push(entity);
                self.ctx.nodes.insert(*id, entity);
            }
            ReplaceWith { id, m } => todo!(),
            ReplacePlaceholder { path, m } => todo!(),
            InsertAfter { id, m } => todo!(),
            InsertBefore { id, m } => todo!(),
            SetAttribute {
                name,
                value,
                id,
                ns,
            } => todo!(),
            SetText { value, id } => todo!(),
            NewEventListener { name, id } => todo!(),
            RemoveEventListener { name, id } => todo!(),
            Remove { id } => todo!(),
            PushRoot { id } => todo!(),
        }
    }

    pub fn apply_mutations(&mut self, mutations: &Mutations) {
        let templates = mutations
            .templates
            .iter()
            .cloned()
            .map(|template| (template.name, template))
            .collect::<HashMap<_, _>>();
        for mutation in &mutations.edits {
            self.apply_mutation(&templates, mutation);
            dbg!(&self.ctx);
        }
    }
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        node {
            background_color: "#ff0000",
            "hello",
        }
    })
}

fn startup(mut gui: GuiParams) {
    let mut vdom = VirtualDom::new(app);
    let mutations = vdom.rebuild();
    dbg!(&mutations);
    gui.apply_mutations(&mutations);
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiContext>()
            .add_startup_system(startup);
    }
}
