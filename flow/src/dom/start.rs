use crate::{Reduce, Render};

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

    let on_mount = |render: &dyn Render, ancestors: Vec<&dyn Render>| {
        // let parent = parent.unwrap();
        // let parent_element = parent.as_element();
        // let element = render.as_element();
        // parent_element.append_child(&element).unwrap();

        // crate::log!("LiView::on_mount");
        // let li_element = web_sys::window()
        //     .unwrap()
        //     .document()
        //     .unwrap()
        //     .get_element_by_id("li")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlLiElement>()
        //     .unwrap();

        // let parent = crate::dom::current_parent_element();
        // parent.append_child(&li_element).unwrap();
    };
    crate::start(model, to_view, on_mount).await;
}
