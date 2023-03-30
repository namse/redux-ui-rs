use super::*;

pub enum Node {
    Single {
        box_render: Box<dyn Render>,
        children: Vec<Node>,
    },
    Multiple {
        nodes: Vec<Node>,
    },
}

impl Node {
    pub fn from_render(render: impl Render + PartialEq + Clone + 'static) -> Self {
        render.on_mount();

        let mut children = vec![];
        update_children(&mut children, render.clone_box());

        Self::Single {
            box_render: Box::new(render),
            children,
        }
    }

    pub fn update(&mut self, render: impl Render + PartialEq + Clone + 'static) {
        let Self::Single{ box_render, children } = self else {
            unreachable!()
        };
        if box_render.as_any().downcast_ref() == Some(&render) {
            println!(" # same props");
            return;
        }

        if box_render.as_any().type_id() != render.type_id() {
            println!(" # different type id");
            render.on_mount();
        }

        println!(" # same type id update props");

        *box_render = render.clone_box();

        update_children(children, Box::new(render));
    }

    fn from_view(view: View) -> Self {
        view.on_mount();

        match view {
            View::Single { box_render } => {
                let children = render_to_children(box_render.clone_box());

                Self::Single {
                    box_render,
                    children,
                }
            }
            View::Multiple { views } => {
                let nodes = views
                    .into_iter()
                    .map(|view| Node::from_view(view))
                    .collect();

                Self::Multiple { nodes }
            }
        }
    }

    fn update_by_view(&mut self, view: View) {
        match (&self, view) {
            (
                Node::Single {
                    box_render,
                    children: _,
                },
                View::Single {
                    box_render: view_box_render,
                },
            ) => {
                if box_render.equals(view_box_render.as_ref()) {
                    println!(" # same props");
                    return;
                }

                if box_render.as_any().type_id() != view_box_render.as_any().type_id() {
                    println!(" # different type id");
                    self.on_unmount();
                    view_box_render.on_mount();
                }
                println!(" # same type id update props");

                let Node::Single {
                    box_render,
                    children,
                } = self else {
                    unreachable!()
                };

                *box_render = view_box_render.clone_box();
                update_children(children, view_box_render);
            }
            (Node::Single { .. }, View::Multiple { views }) => {
                self.on_unmount();

                let nodes = views
                    .into_iter()
                    .map(|view| Node::from_view(view))
                    .collect();

                *self = Node::Multiple { nodes };
            }
            (Node::Multiple { nodes: _ }, View::Single { box_render }) => {
                self.on_unmount();
                box_render.on_mount();

                let children = render_to_children(box_render.clone_box());

                *self = Node::Single {
                    box_render,
                    children,
                };
            }
            (Node::Multiple { .. }, View::Multiple { views }) => {
                let Node::Multiple { nodes } = self else {
                    unreachable!()
                };

                let max_index = std::cmp::max(nodes.len(), views.len());

                for (index, view) in views.into_iter().enumerate() {
                    let node = nodes.get_mut(index);
                    match node {
                        Some(node) => {
                            node.update_by_view(view);
                        }
                        None => {
                            nodes.push(Node::from_view(view));
                        }
                    }
                }

                for _ in max_index..nodes.len() {
                    let node = nodes.pop().unwrap();
                    node.on_unmount();
                }
            }
        }
    }

    fn on_unmount(&self) {
        match self {
            Node::Single {
                box_render,
                children,
            } => {
                for child in children {
                    child.on_unmount();
                }
                box_render.on_unmount();
            }
            Node::Multiple { nodes } => {
                for node in nodes {
                    node.on_unmount();
                }
            }
        }
    }
}

fn update_children(children: &mut Vec<Node>, render: Box<dyn Render>) {
    #[allow(deprecated)]
    let views: Vec<View> = render_to_views(render);

    let max_index = std::cmp::max(children.len(), views.len());

    for (index, view) in views.into_iter().enumerate() {
        let child = children.get_mut(index);
        match child {
            Some(child) => {
                child.update_by_view(view);
            }
            None => {
                children.push(Node::from_view(view));
            }
        }
    }

    for _ in max_index..children.len() {
        let child = children.pop().unwrap();
        child.on_unmount();
    }
}

fn render_to_views(render: Box<dyn Render>) -> Vec<View> {
    #[allow(deprecated)]
    match render.render() {
        View::Single { box_render } => vec![View::Single { box_render }],
        View::Multiple { views } => views,
    }
}

fn render_to_children(render: Box<dyn Render>) -> Vec<Node> {
    let views = render_to_views(render);
    views
        .into_iter()
        .map(|view| Node::from_view(view))
        .collect()
}

// fn test(render: Box<dyn Render>) {
//     testa(&render);
// }
// fn testa(render: &dyn AnyEqual) {}
