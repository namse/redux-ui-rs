use crate::*;
use render_tree::{Node, RenderTree};

pub async fn start<'a, Model: Reduce, View: Render + PartialEq + Clone + 'static>(
    mut model: Model,
    to_view: impl Fn(&Model) -> View,
    on_mount: &dyn Fn(&Node, &Vec<&Node>),
) {
    let mut render_tree: Option<RenderTree> = None;
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

fn update_view<'a>(
    render_tree: &mut Option<RenderTree>,
    view: impl Render + PartialEq + Clone + 'static,
    on_mount: &dyn Fn(&Node, &Vec<&Node>),
) {
    println!("update_view");
    match render_tree.as_mut() {
        Some(render_tree) => {
            // render_tree.update(view, &on_mount);
        }
        None => {
            *render_tree = Some(RenderTree::from_render(view, &on_mount));
        }
    }
}

async fn get_event() -> Box<dyn std::any::Any> {
    Box::new(())
}
