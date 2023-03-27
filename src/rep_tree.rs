use super::*;

pub struct Node {
    view: View,
    children: Vec<Node>,
}

impl Node {
    pub fn from_render(render: Box<dyn Render>) -> Self {
        render.on_mount();

        let rep = render.clone_box().render();
        let view = View { render };
        let mut children = vec![];
        update_children(&mut children, rep);

        Self { view, children }
    }

    pub fn update(&mut self, render: Box<dyn Render>) {
        if self.view.render.eq(&render) {
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

        self.view = View { render };

        update_children(&mut self.children, rep);
    }

    fn from_view(view: View) -> Self {
        view.render.on_mount();

        let children = rep_to_children(view.render.render());

        Self { view, children }
    }
}

fn update_children(children: &mut Vec<Node>, rep: Rep) {
    let max_index = std::cmp::max(children.len(), rep.views.len());

    for (index, view) in rep.views.into_iter().enumerate() {
        let child = children.get_mut(index);
        match child {
            Some(child) => {
                child.update(view.render);
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
