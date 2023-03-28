use super::*;

pub struct Node {
    view: View,
    children: Vec<Node>,
}

impl Node {
    pub fn from_render(render: impl Render + PartialEq + Clone + 'static) -> Self {
        render.on_mount();

        let rep = render.clone_box().render();
        let view = View {
            render: Box::new(render),
        };
        let mut children = vec![];
        update_children(&mut children, rep);

        Self { view, children }
    }

    pub fn update(&mut self, render: impl Render + PartialEq + Clone + 'static) {
        if self.view.render.as_any().downcast_ref() == Some(&render) {
            println!(" # same props");
            return;
        }

        if self.view.render.as_any().type_id() != render.type_id() {
            println!(" # different type id");
            render.on_mount();
        }

        println!(" # same type id update props");

        let rep = render.clone_box().render();

        self.view = View {
            render: Box::new(render),
        };

        update_children(&mut self.children, rep);
    }

    fn from_view(view: View) -> Self {
        view.render.on_mount();

        let children = rep_to_children(view.render.clone_box().render());

        Self { view, children }
    }

    fn update_by_view(&mut self, view: View) {
        // if self.view.render.as_any().downcast_ref() == view.render.as_any().downcast_ref() {
        //     return;
        // }

        if self.view.render.as_any().type_id() != view.render.as_any().type_id() {
            println!(
                "self.view.render.type_id() {:?}",
                self.view.render.type_id()
            );
            println!("view.render.type_id() {:?}", view.render.type_id());
            view.render.on_mount();
        }

        let rep = view.render.clone_box().render();

        self.view = view;

        update_children(&mut self.children, rep);
    }
}

fn update_children(children: &mut Vec<Node>, rep: Rep) {
    let max_index = std::cmp::max(children.len(), rep.views.len());

    for (index, view) in rep.views.into_iter().enumerate() {
        let child = children.get_mut(index);
        match child {
            Some(child) => {
                child.update_by_view(view);
            }
            None => {
                children.push(Node::from_view(view.clone()));
            }
        }
    }

    for _ in max_index..children.len() {
        let child = children.pop().unwrap();
        child.view.render.on_unmount();
    }
}

fn rep_to_children(rep: Rep) -> Vec<Node> {
    rep.views
        .into_iter()
        .map(|view| Node::from_view(view))
        .collect()
}
