mod reduce;
mod render;
mod rep_tree;
mod start;

use reduce::Reduce;
use render::{render, Element, Render};
use start::start;
use std::{any::Any, fmt::Debug};

fn main() {
    start(
        TodoAppModel {
            text_input: text_input::Model::new(),
            todos: TodoModel { todos: vec![] },
            visibility_filter: VisibilityFilterModel {
                visibility_filter: VisibilityFilter::ShowAll,
            },
        },
        |TodoAppModel {
             todos,
             visibility_filter,
             text_input,
         }| {
            TodoAppView {
                todos: todos.todos.clone(),
                visibility_filter: visibility_filter.visibility_filter,
                text_input: text_input.map_to_view(),
            }
        },
    )
}

#[derive(PartialEq, Clone)]
struct Todo {
    text: String,
    completed: bool,
}
struct TodoModel {
    todos: Vec<Todo>,
}

#[derive(PartialEq, Debug)]
enum TodoEvent {
    AddTodo { text: String },
    ToggleTodo { index: usize },
    Nothing,
}

impl Reduce for TodoModel {
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
    pub struct Model {}

    impl Model {
        pub fn new() -> Self {
            Self {}
        }
        pub fn map_to_view(&self) -> View {
            View {}
        }
    }

    pub enum Event {}

    impl super::Reduce for Model {
        fn reduce(self, event: &dyn std::any::Any) -> Self {
            #[allow(unused_variables)]
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
        fn render(self: Box<Self>) -> super::Element {
            render(())
        }
    }
}

struct TodoAppModel {
    todos: TodoModel,
    visibility_filter: VisibilityFilterModel,
    text_input: text_input::Model,
}

impl Reduce for TodoAppModel {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        TodoAppModel {
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
    fn render(self: Box<Self>) -> Element {
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
    fn render(self: Box<Self>) -> Element {
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
    fn render(self: Box<Self>) -> Element {
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

fn text(text: impl ToString) -> Element {
    println!("text: {:?}", text.to_string());
    render(())
}

#[derive(PartialEq, Clone)]
struct VisibilityFilterView {
    visibility_filter: VisibilityFilter,
}

impl Render for VisibilityFilterView {
    fn render(self: Box<Self>) -> Element {
        render(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum VisibilityFilter {
    ShowAll,
    ShowCompleted,
}

struct VisibilityFilterModel {
    visibility_filter: VisibilityFilter,
}

enum VisibilityFilterEvent {
    SetVisibilityFilter(VisibilityFilter),
}

impl Reduce for VisibilityFilterModel {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        if let Some(event) = event.downcast_ref::<VisibilityFilterEvent>() {
            match event {
                VisibilityFilterEvent::SetVisibilityFilter(visibility_filter) => {
                    VisibilityFilterModel {
                        visibility_filter: *visibility_filter,
                    }
                }
            }
        } else {
            self
        }
    }
}
