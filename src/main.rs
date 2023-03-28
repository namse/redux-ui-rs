#![feature(trait_upcasting)]

mod reduce;
mod rep_tree;

use reduce::Reduce;
use std::{
    any::Any,
    fmt::Debug,
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
    let view = to_view(&state);
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
        state = state.reduce(event.as_ref());

        let view = to_view(&state);
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

fn get_event() -> Box<dyn std::any::Any> {
    Box::new(TodoEvent::AddTodo {
        text: "Hello".to_string(),
    })
}

pub enum View {
    Single { box_render: Box<dyn Render> },
    Multiple { views: Vec<View> },
}

impl View {
    fn on_mount(&self) {
        match self {
            View::Single { box_render } => box_render.on_mount(),
            View::Multiple { views } => {
                for view in views {
                    view.on_mount();
                }
            }
        }
    }
    fn on_unmount(&self) {
        match self {
            View::Single { box_render } => box_render.on_unmount(),
            View::Multiple { views } => {
                for view in views {
                    view.on_unmount();
                }
            }
        }
    }
}

impl Clone for View {
    fn clone(&self) -> Self {
        match self {
            View::Single { box_render } => View::Single {
                box_render: box_render.clone_box(),
            },
            View::Multiple { views } => View::Multiple {
                views: views.clone(),
            },
        }
    }
}

pub trait Render: AnyEqual + CloneBox {
    #[deprecated(note = "Please do not use this method.")]
    fn render(self: Box<Self>) -> View;
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

#[macro_export]
macro_rules! __rust_force_expr {
    ($e:expr) => {
        $e
    };
}

trait IntoView {
    fn into_rep(self) -> View;
}

impl IntoView for () {
    fn into_rep(self) -> View {
        View::Multiple { views: vec![] }
    }
}

impl<T0> IntoView for T0
where
    T0: Render + 'static,
{
    fn into_rep(self) -> View {
        View::Multiple {
            views: vec![View::Single {
                box_render: Box::new(self),
            }],
        }
    }
}

impl<T0, T1> IntoView for (T0, T1)
where
    T0: Render + 'static,
    T1: Render + 'static,
{
    fn into_rep(self) -> View {
        let (t0, t1) = self;
        View::Multiple {
            views: vec![
                View::Single {
                    box_render: Box::new(t0),
                },
                View::Single {
                    box_render: Box::new(t1),
                },
            ],
        }
    }
}

impl<T0, T1, T2> IntoView for (T0, T1, T2)
where
    T0: Render + 'static,
    T1: Render + 'static,
    T2: Render + 'static,
{
    fn into_rep(self) -> View {
        let (t0, t1, t2) = self;

        View::Multiple {
            views: vec![
                View::Single {
                    box_render: Box::new(t0),
                },
                View::Single {
                    box_render: Box::new(t1),
                },
                View::Single {
                    box_render: Box::new(t2),
                },
            ],
        }
    }
}

impl<T: Render + 'static> IntoView for Vec<T> {
    fn into_rep(self) -> View {
        View::Multiple {
            views: self
                .into_iter()
                .map(|t| View::Single {
                    box_render: Box::new(t),
                })
                .collect(),
        }
    }
}

fn render(into_rep: impl IntoView) -> View {
    into_rep.into_rep()
}

#[derive(PartialEq, Clone)]
struct Todo {
    text: String,
    completed: bool,
}
struct TodoState {
    todos: Vec<Todo>,
}

#[derive(PartialEq, Debug)]
enum TodoEvent {
    AddTodo { text: String },
    ToggleTodo { index: usize },
    Nothing,
}

impl Reduce for TodoState {
    fn reduce(mut self, event: &dyn std::any::Any) -> Self {
        if let Some(event) = event.downcast_ref::<TodoEvent>() {
            match event {
                TodoEvent::AddTodo { text } => {
                    println!("Add todo: {}", text);
                    self.todos.last_mut().map(|todo| todo.text += "1");
                    self.todos.push(Todo {
                        text: text.clone(),
                        completed: false,
                    });
                }
                TodoEvent::ToggleTodo { index } => {
                    self.todos[*index].completed = !self.todos[*index].completed;
                }
                TodoEvent::Nothing => {}
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
        fn render(self: Box<Self>) -> super::View {
            render(())
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
    fn render(self: Box<Self>) -> View {
        println!("TodoAppView render called");
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
            self.text_input,
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
    fn render(self: Box<Self>) -> View {
        let mut elements = vec![];

        for (index, todo) in self.todos.iter().enumerate() {
            elements.push(TodoView {
                text: todo.text.clone(),
                completed: todo.completed,
                index,
            });
        }

        println!("TodoListView rendered: {} todos", elements.len());

        render(elements)
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
    fn render(self: Box<Self>) -> View {
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

fn text(text: impl ToString) -> View {
    println!("text: {:?}", text.to_string());
    render(())
}

impl View {
    fn event(self, build: impl FnOnce(&mut EventBuilder)) -> View {
        render(())
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
    fn render(self: Box<Self>) -> View {
        render(())
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
