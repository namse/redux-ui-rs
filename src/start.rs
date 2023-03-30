use crate::*;

pub fn start<Model: Reduce, View: Render + PartialEq + Clone + 'static>(
    mut model: Model,
    to_view: impl Fn(&Model) -> View,
) {
    let mut rep_tree: Option<rep_tree::Node> = None;
    let view = to_view(&model);
    update_view(&mut rep_tree, view);

    let events: Vec<_> = vec![
        Box::new(TodoEvent::AddTodo {
            text: "Hello".to_string(),
        }),
        Box::new(TodoEvent::AddTodo {
            text: "World".to_string(),
        }),
        Box::new(TodoEvent::Nothing),
        Box::new(TodoEvent::Nothing),
    ];
    for event in events {
        println!("\n\n# event: {:?}", event);

        // let event = get_event();
        model = model.reduce(event.as_ref());

        let view = to_view(&model);
        update_view(&mut rep_tree, view);
    }
}

fn update_view(
    rep_tree: &mut Option<rep_tree::Node>,
    view: impl Render + PartialEq + Clone + 'static,
) {
    println!("update_view");
    match rep_tree.as_mut() {
        Some(rep_tree) => {
            rep_tree.update(view);
        }
        None => {
            *rep_tree = Some(rep_tree::Node::from_render(view));
        }
    }
}

#[allow(dead_code)]
fn get_event() -> Box<dyn std::any::Any> {
    Box::new(TodoEvent::AddTodo {
        text: "Hello".to_string(),
    })
}
