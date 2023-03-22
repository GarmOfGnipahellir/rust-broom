use bevy::{
    ecs::system::SystemParam,
    prelude::{
        App, AppTypeRegistry, BackgroundColor, BuildWorldChildren, Commands, Entity, FromWorld,
        Mut, NodeBundle, Plugin, Reflect, ReflectComponent, Res, ResMut, Resource, Size, Style,
        Val, World,
    },
    utils::HashMap,
};
use dioxus::{
    core::{ElementId, Mutation, Mutations},
    prelude::*,
};

pub mod dioxus_elements {
    pub struct node;
    impl node {
        pub const TAG_NAME: &'static str = "Node";
        pub const NAME_SPACE: Option<&'static str> = None;

        pub const background_color: (&'static str, Option<&'static str>, bool) =
            ("background_color", None, false);
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
    pub type_registry: Res<'w, AppTypeRegistry>,
}

impl GuiContext {
    pub fn spawn_template_node(&mut self, world: &mut World, node: &TemplateNode) -> Entity {
        use dioxus::core::TemplateNode::*;
        match node {
            Element {
                tag,
                namespace,
                attrs,
                children,
            } => {
                match tag {
                    "node" => 
                    _ => {},
                }
            }
            Text { text } => todo!(),
            Dynamic { id } => todo!(),
            DynamicText { id } => todo!(),
        }
        world.spawn(NodeBundle::default()).id()
    }

    pub fn apply_mutation(
        &mut self,
        world: &mut World,
        templates: &HashMap<&str, Template>,
        mutation: &Mutation,
    ) {
        use dioxus::core::Mutation::*;
        match mutation {
            AppendChildren { id, m } => {
                let parent = *self.nodes.get(id).unwrap();
                for _ in 0..*m {
                    let child = self.stack.pop().unwrap();
                    world.entity_mut(parent).add_child(child);
                }
            }
            AssignId { path, id } => todo!(),
            CreatePlaceholder { id } => todo!(),
            CreateTextNode { value, id } => todo!(),
            HydrateText { path, value, id } => todo!(),
            LoadTemplate { name, index, id } => {
                let template = templates.get(name).unwrap();
                let root = template.roots.get(*index).unwrap();
                let entity = self.spawn_template_node(world, root);
                self.stack.push(entity);
                self.nodes.insert(*id, entity);
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

    pub fn apply_mutations(&mut self, world: &mut World, mutations: &Mutations) {
        let templates = mutations
            .templates
            .iter()
            .cloned()
            .map(|template| (template.name, template))
            .collect::<HashMap<_, _>>();
        for mutation in &mutations.edits {
            self.apply_mutation(world, &templates, mutation);
            dbg!(&self);
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

fn startup(/*mut gui: GuiParams, tr: Res<AppTypeRegistry>,*/ world: &mut World) {
    world.resource_scope(|world: &mut World, mut gui: Mut<GuiContext>| {
        let mut vdom = VirtualDom::new(app);
        let mutations = vdom.rebuild();
        dbg!(&mutations);
        gui.apply_mutations(world, &mutations);
    });

    world.resource_scope(|world: &mut World, tr: Mut<AppTypeRegistry>| {
        let tr = tr.read();
        let bgc_refl: Box<dyn Reflect> = Box::new(BackgroundColor::default());
        let bgc_reg = tr.get_with_short_name("BackgroundColor").unwrap();
        let bgc_refl_comp = bgc_reg.data::<ReflectComponent>().unwrap();
        let mut ent = world.spawn_empty();
        bgc_refl_comp.insert(&mut ent, bgc_refl.as_reflect());
    })
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiContext>()
            .add_startup_system(startup);
    }
}
