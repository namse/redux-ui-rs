use super::*;

pub struct Node {
    view: View,
    children: Vec<Node>,
}

impl Node {
    pub fn from_render(render: impl Render + PartialEq + 'static) -> Self {
        render.on_mount();

        let rep = render.render();
        let view = View {
            render: Arc::new(render),
        };
        let mut children = vec![];
        update_children(&mut children, rep);

        Self { view, children }
    }

    pub fn update(&mut self, render: impl Render + PartialEq + 'static) {
        if self.view.render.as_any().downcast_ref() == Some(&render) {
            return;
        }

        if self.view.render.type_id() != render.type_id() {
            println!(
                "self.view.render.type_id() {:?}",
                self.view.render.type_id()
            );
            println!("render.type_id() {:?}", render.type_id());
            render.on_mount();
        }

        let rep = render.render();

        self.view = View {
            render: Arc::new(render),
        };

        update_children(&mut self.children, rep);
    }

    fn from_view(view: View) -> Self {
        view.render.on_mount();

        let children = rep_to_children(view.render.render());

        Self { view, children }
    }

    fn update_by_view(&mut self, view: View) {
        if Arc::ptr_eq(&self.view.render, &view.render) {
            return;
        }

        if self.view.render.as_any().type_id() != view.render.as_any().type_id() {
            println!(
                "self.view.render.type_id() {:?}",
                self.view.render.type_id()
            );
            println!("view.render.type_id() {:?}", view.render.type_id());
            view.render.on_mount();
        }

        let rep = view.render.render();

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
