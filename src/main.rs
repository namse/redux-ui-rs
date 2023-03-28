mod reduce;
mod rep_tree;

use reduce::Reduce;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

fn main() {
    let app_state = TodoAppState {
        text_input: text_input::State::new(),
        todos: TodoState { todos: vec![] },
        visibility_filter: VisibilityFilterState {
            visibility_filter: VisibilityFilter::ShowAll,
        },
    };

    run(app_state, |app_state| {
        let TodoAppState {
            todos,
            visibility_filter,
            text_input,
        } = app_state;

        TodoAppView {
            todos: todos.todos.clone(),
            visibility_filter: visibility_filter.visibility_filter,
            text_input: text_input.map_to_view(),
        }
    })
}

fn run<State: Reduce, View: Render + PartialEq + Clone + 'static>(
    mut state: State,
    to_view: impl Fn(&State) -> View,
) {
    let mut rep_tree: Option<rep_tree::Node> = None;
    let events: Vec<Box<dyn std::any::Any>> = vec![
        Box::new(TodoEvent::AddTodo {
            text: "Hello".to_string(),
        }),
        Box::new(TodoEvent::AddTodo {
            text: "World".to_string(),
        }),
        Box::new(1),
        Box::new(1),
    ];
    for event in events {
        let view = to_view(&state);
        update_view(&mut rep_tree, view);

        // let event = get_event();
        state = state.reduce(event.as_ref());
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

fn get_event() -> Box<dyn std::any::Any> {
    Box::new(TodoEvent::AddTodo {
        text: "Hello".to_string(),
    })
}

#[derive(Clone)]
pub struct Rep {
    views: Vec<View>,
}

pub struct View {
    render: Box<dyn Render>,
}

impl Clone for View {
    fn clone(&self) -> Self {
        View {
            render: self.render.clone_box(),
        }
    }
}

pub trait Render: AnyEqual + CloneBox {
    fn render(self: Box<Self>) -> Rep;
    fn on_mount(&self) {}
    fn on_unmount(&self) {}
}

pub trait AnyEqual {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, _: &dyn AnyEqual) -> bool;
}

impl<S: 'static + PartialEq> AnyEqual for S {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn AnyEqual) -> bool {
        other
            .as_any()
            .downcast_ref::<S>()
            .map_or(false, |a| self == a)
    }
}

pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn Render>;
}

impl<S: 'static + Clone + Render> CloneBox for S {
    fn clone_box(&self) -> Box<dyn Render> {
        Box::new(Clone::clone(self))
    }
}

fn div(children: Vec<View>) -> Rep {
    Rep { views: children }
}

fn views(children: Vec<View>) -> Rep {
    Rep { views: children }
}

#[macro_export]
macro_rules! __rust_force_expr {
    ($e:expr) => {
        $e
    };
}

trait IntoRep {
    fn into_rep(self) -> Rep;
}

impl<T0> IntoRep for T0
where
    T0: Render + 'static,
{
    fn into_rep(self) -> Rep {
        Rep {
            views: vec![View {
                render: Box::new(self),
            }],
        }
    }
}

impl<T0, T1> IntoRep for (T0, T1)
where
    T0: Render + 'static,
    T1: Render + 'static,
{
    fn into_rep(self) -> Rep {
        let (t0, t1) = self;
        Rep {
            views: vec![
                View {
                    render: Box::new(t0),
                },
                View {
                    render: Box::new(t1),
                },
            ],
        }
    }
}

impl<T0, T1, T2> IntoRep for (T0, T1, T2)
where
    T0: Render + 'static,
    T1: Render + 'static,
    T2: Render + 'static,
{
    fn into_rep(self) -> Rep {
        let (t0, t1, t2) = self;
        Rep {
            views: vec![
                View {
                    render: Box::new(t0),
                },
                View {
                    render: Box::new(t1),
                },
                View {
                    render: Box::new(t2),
                },
            ],
        }
    }
}

fn render(into_rep: impl IntoRep) -> Rep {
    into_rep.into_rep()
}

fn view<R: Render + PartialEq + 'static>(render: R) -> View {
    View {
        render: Box::new(render),
    }
}

#[derive(PartialEq, Clone)]
struct Todo {
    text: String,
    completed: bool,
}
struct TodoState {
    todos: Vec<Todo>,
}

enum TodoEvent {
    AddTodo { text: String },
    ToggleTodo { index: usize },
}

impl Reduce for TodoState {
    fn reduce(mut self, event: &dyn std::any::Any) -> Self {
        if let Some(event) = event.downcast_ref::<TodoEvent>() {
            match event {
                TodoEvent::AddTodo { text } => {
                    println!("Add todo: {}", text);
                    self.todos.push(Todo {
                        text: text.clone(),
                        completed: false,
                    });
                }
                TodoEvent::ToggleTodo { index } => {
                    self.todos[*index].completed = !self.todos[*index].completed;
                }
            }
        }

        self
    }
}

mod text_input {
    use super::*;
    pub struct State {}

    impl State {
        pub fn new() -> Self {
            Self {}
        }
        pub fn map_to_view(&self) -> View {
            View {}
        }
    }

    pub enum Event {}

    impl super::Reduce for State {
        fn reduce(self, event: &dyn std::any::Any) -> Self {
            if let Some(event) = event.downcast_ref::<Event>() {
                todo!()
            } else {
                self
            }
        }
    }
    #[derive(PartialEq, Clone)]
    pub struct View {}

    impl Render for View {
        fn render(self: Box<Self>) -> Rep {
            div(vec![])
        }
    }
}

struct TodoAppState {
    todos: TodoState,
    visibility_filter: VisibilityFilterState,
    text_input: text_input::State,
}

impl Reduce for TodoAppState {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        TodoAppState {
            todos: self.todos.reduce(event),
            visibility_filter: self.visibility_filter.reduce(event),
            text_input: self.text_input.reduce(event),
        }
    }
}

#[derive(PartialEq, Clone)]
struct TodoAppView {
    todos: Vec<Todo>,
    visibility_filter: VisibilityFilter,
    text_input: text_input::View,
}

impl Render for TodoAppView {
    fn render(self: Box<Self>) -> Rep {
        let filtered_todos = self
            .todos
            .iter()
            .filter(|todo| match self.visibility_filter {
                VisibilityFilter::ShowAll => true,
                VisibilityFilter::ShowCompleted => todo.completed,
            })
            .cloned()
            .collect::<Vec<_>>();

        println!("todos: {:?}", self.todos.len());
        println!("filtered_todos: {:?}", filtered_todos.len());

        render((
            TodoListView {
                todos: filtered_todos,
            },
            VisibilityFilterView {
                visibility_filter: self.visibility_filter,
            },
            self.text_input.clone(),
        ))
    }
    fn on_mount(&self) {
        println!("TodoAppView mounted");
    }
    fn on_unmount(&self) {
        println!("TodoAppView unmounted");
    }
}
#[derive(PartialEq, Clone)]
struct TodoListView {
    todos: Vec<Todo>,
}

impl Render for TodoListView {
    fn render(self: Box<Self>) -> Rep {
        let mut elements = vec![];

        for (index, todo) in self.todos.iter().enumerate() {
            elements.push(view(TodoView {
                text: todo.text.clone(),
                completed: todo.completed,
                index,
            }));
        }

        println!("TodoListView rendered: {} todos", elements.len());

        div(elements)
    }
    fn on_mount(&self) {
        println!("TodoListView mounted");
    }
    fn on_unmount(&self) {
        println!("TodoListView unmounted");
    }
}

#[derive(PartialEq, Clone)]
struct TodoView {
    text: String,
    completed: bool,
    index: usize,
}

impl Render for TodoView {
    fn render(self: Box<Self>) -> Rep {
        text(&self.text).event(|build| {
            let index = self.index;
            build.on_click_fn(move |_| Some(TodoEvent::ToggleTodo { index }));

            build.on_click(TodoEvent::ToggleTodo { index: self.index });
        })
    }
    fn on_mount(&self) {
        println!("TodoView mounted");
    }
    fn on_unmount(&self) {
        println!("TodoView unmounted");
    }
}

fn text(text: impl ToString) -> Rep {
    println!("text: {:?}", text.to_string());
    div(vec![])
}

impl Rep {
    fn event(self, build: impl FnOnce(&mut EventBuilder)) -> Rep {
        div(vec![])
    }
}

struct EventBuilder {}

impl EventBuilder {
    fn on_click_fn<Event: std::any::Any>(
        &mut self,
        handler: impl Fn(ClickEvent) -> Option<Event> + 'static,
    ) -> Self {
        todo!()
    }
    fn on_click<Event: std::any::Any>(&mut self, event: Event) -> Self {
        todo!()
    }
}

struct ClickEvent {}

#[derive(PartialEq, Clone)]
struct VisibilityFilterView {
    visibility_filter: VisibilityFilter,
}

impl Render for VisibilityFilterView {
    fn render(self: Box<Self>) -> Rep {
        div(vec![])
    }
}

#[derive(Clone, Copy, PartialEq)]
enum VisibilityFilter {
    ShowAll,
    ShowCompleted,
}

struct VisibilityFilterState {
    visibility_filter: VisibilityFilter,
}

enum VisibilityFilterEvent {
    SetVisibilityFilter(VisibilityFilter),
}

impl Reduce for VisibilityFilterState {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        if let Some(event) = event.downcast_ref::<VisibilityFilterEvent>() {
            match event {
                VisibilityFilterEvent::SetVisibilityFilter(visibility_filter) => {
                    VisibilityFilterState {
                        visibility_filter: *visibility_filter,
                    }
                }
            }
        } else {
            self
        }
    }
}
