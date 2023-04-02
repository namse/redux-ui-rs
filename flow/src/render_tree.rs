use super::*;
use std::any::Any;

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
    pub fn from_render(
        render: impl Render + PartialEq + Clone + 'static,
        on_mount: impl Fn(&dyn Render, Vec<&dyn Render>),
    ) -> Self {
        let mut ancestors: Vec<&dyn Render> = vec![];
        render.on_mount();
        on_mount(&render, &mut ancestors);

        ancestors.push(&render);

        let mut children = vec![];
        update_children(&mut children, render.clone_box(), &mut ancestors);

        Self::Single {
            box_render: Box::new(render),
            children,
        }
    }

    pub fn update(
        &mut self,
        render: impl Render + PartialEq + Clone + 'static,
        on_mount: impl Fn(&dyn Render, Option<&dyn Render>),
    ) {
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

    fn from_element(element: Element) -> Self {
        element.on_mount();

        match element {
            Element::Single { box_render } => {
                let children = render_to_children(box_render.clone_box());

                Self::Single {
                    box_render,
                    children,
                }
            }
            Element::Multiple { elements } => {
                let nodes = elements
                    .into_iter()
                    .map(|element| Node::from_element(element))
                    .collect();

                Self::Multiple { nodes }
            }
        }
    }

    fn update_by_element(&mut self, element: Element) {
        match (&self, element) {
            (
                Node::Single {
                    box_render,
                    children: _,
                },
                Element::Single {
                    box_render: element_box_render,
                },
            ) => {
                if box_render.equals(element_box_render.as_ref()) {
                    println!(" # same props");
                    return;
                }

                if box_render.as_any().type_id() != element_box_render.as_any().type_id() {
                    println!(" # different type id");
                    self.on_unmount();
                    element_box_render.on_mount();
                }
                println!(" # same type id update props");

                let Node::Single {
                    box_render,
                    children,
                } = self else {
                    unreachable!()
                };

                *box_render = element_box_render.clone_box();
                update_children(children, element_box_render);
            }
            (Node::Single { .. }, Element::Multiple { elements }) => {
                self.on_unmount();

                let nodes = elements
                    .into_iter()
                    .map(|element| Node::from_element(element))
                    .collect();

                *self = Node::Multiple { nodes };
            }
            (Node::Multiple { nodes: _ }, Element::Single { box_render }) => {
                self.on_unmount();
                box_render.on_mount();

                let children = render_to_children(box_render.clone_box());

                *self = Node::Single {
                    box_render,
                    children,
                };
            }
            (Node::Multiple { .. }, Element::Multiple { elements }) => {
                let Node::Multiple { nodes } = self else {
                    unreachable!()
                };

                let max_index = std::cmp::max(nodes.len(), elements.len());

                for (index, element) in elements.into_iter().enumerate() {
                    let node = nodes.get_mut(index);
                    match node {
                        Some(node) => {
                            node.update_by_element(element);
                        }
                        None => {
                            nodes.push(Node::from_element(element));
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
    let elements: Vec<Element> = render_to_elements(render);

    let max_index = std::cmp::max(children.len(), elements.len());

    for (index, element) in elements.into_iter().enumerate() {
        let child = children.get_mut(index);
        match child {
            Some(child) => {
                child.update_by_element(element);
            }
            None => {
                children.push(Node::from_element(element));
            }
        }
    }

    for _ in max_index..children.len() {
        let child = children.pop().unwrap();
        child.on_unmount();
    }
}

fn render_to_elements(render: Box<dyn Render>) -> Vec<Element> {
    #[allow(deprecated)]
    match render.render() {
        Element::Single { box_render } => vec![Element::Single { box_render }],
        Element::Multiple { elements } => elements,
    }
}

fn render_to_children(render: Box<dyn Render>) -> Vec<Node> {
    let elements = render_to_elements(render);
    elements
        .into_iter()
        .map(|element| Node::from_element(element))
        .collect()
}

// fn test(render: Box<dyn Render>) {
//     testa(&render);
// }
// fn testa(render: &dyn AnyEqual) {}
