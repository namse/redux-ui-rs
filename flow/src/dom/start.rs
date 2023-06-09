use crate::{render_tree::Node, LiView, Reduce, Render};
use wasm_bindgen::JsCast;

pub async fn start_dom<Model: Reduce, View: Render + PartialEq + Clone + 'static>(
    root_id: impl ToString,
    mut model: Model,
    to_view: impl Fn(&Model) -> View,
) {
    let root_id = root_id.to_string();
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(&root_id)
        .expect(format!("Could not find element with id: {}", root_id).as_str());

    let on_mount = |node: &Node, ancestors: &Vec<&Node>| {
        crate::log!("on_mount");

        if let Some(_) = node.box_render.as_any().downcast_ref::<LiView>() {
            crate::log!("LiView::on_mount");
            let li_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("li")
                .unwrap()
                .dyn_into::<web_sys::HtmlLiElement>()
                .unwrap();

            // TODO: Set style

            let parent = find_dom_parent(&ancestors).unwrap_or_else(|| root.clone());
            parent.append_child(&li_element).unwrap();

            *node.platform_data.lock().unwrap() = Some(Box::new(li_element));
        }
    };
    crate::start(model, to_view, &on_mount).await;
}

fn find_dom_parent(ancestors: &[&Node]) -> Option<web_sys::Element> {
    for near_ancestor in ancestors.iter().rev() {
        let platform_data = near_ancestor.platform_data.lock().unwrap();
        if let Some(platform_data) = platform_data.as_ref() {
            return Some(
                platform_data
                    .downcast_ref::<web_sys::Element>()
                    .unwrap()
                    .clone(),
            );
        }
    }

    None
}
