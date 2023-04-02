use crate::*;

pub async fn start<Model: Reduce, View: Render + PartialEq + Clone + 'static>(
    mut model: Model,
    to_view: impl Fn(&Model) -> View,
    on_mount: impl Fn(&dyn Render, Option<&dyn Render>),
) {
    let mut render_tree: Option<render_tree::Node> = None;
    let view = to_view(&model);
    update_view(&mut render_tree, view, on_mount);

    // loop {
    //     let event = get_event().await;
    //     println!("\n\n# event: {:?}", event);

    //     model = model.reduce(event.as_ref());

    //     let view = to_view(&model);
    //     update_view(&mut render_tree, view);
    // }
}

fn update_view(
    render_tree: &mut Option<render_tree::Node>,
    view: impl Render + PartialEq + Clone + 'static,
    on_mount: impl Fn(&dyn Render, Option<&dyn Render>),
) {
    println!("update_view");
    match render_tree.as_mut() {
        Some(render_tree) => {
            render_tree.update(view, on_mount);
        }
        None => {
            *render_tree = Some(render_tree::Node::from_render(view, on_mount));
        }
    }
}

async fn get_event() -> Box<dyn std::any::Any> {
    Box::new(())
}
