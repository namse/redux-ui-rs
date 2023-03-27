use std::{
    any::Any,
    sync::{Arc, Mutex},
};

fn main() {
    println!("Hello, world!");
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

        TodoAppProps {
            todos: todos.todos.clone(),
            visibility_filter: visibility_filter.visibility_filter,
            text_input: text_input.map_to_props(),
        }
    })
}

fn run<State: Reduce, Props: Render>(mut state: State, to_props: impl Fn(&State) -> Props) {
    loop {
        let event = get_event();
        state = state.reduce(&event);

        let props = to_props(&state);
        let element = props.render();
        draw_element(element);
    }
}

fn get_event() -> Box<dyn std::any::Any> {
    todo!()
}

fn draw_element(element: Element) {
    todo!()
}

trait Reduce {
    fn reduce(self, event: &dyn std::any::Any) -> Self;
}

#[derive(Clone)]
struct Element {
    views: Vec<View>,
}

trait Render: PartialEq {
    fn render(&self) -> Element;
    fn on_mount(&self) {}
}

fn div(children: Vec<View>) -> Element {
    Element { views: children }
}

fn views(children: Vec<View>) -> Element {
    Element { views: children }
}

#[macro_export]
macro_rules! __rust_force_expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! render {
    () => (
        $crate::__rust_force_expr!($crate::views($crate::vec::Vec::new()))
    );
    ($($x:expr),+ $(,)?) => (
        $crate::__rust_force_expr!($crate::views(
            Box::new([$($crate::view($x)),+]).to_vec()
        ))
    );
}

thread_local! {
    static VIEW_CACHE: Mutex<Vec<View>> = Mutex::new(Vec::new());
}

fn view<R: Render + 'static>(render: R) -> View {
    VIEW_CACHE.with(move |cache| {
        let mut cache = cache.lock().unwrap();
        match cache
            .iter()
            .find(|cached| cached.render.downcast_ref() == Some(&render))
        {
            Some(cached) => {
                let view = cached.clone();
                view
            }
            None => {
                let view = View {
                    render: Arc::new(render),
                };

                cache.push(view.clone());

                view
            }
        }
    })
}

#[derive(Clone)]
struct View {
    render: Arc<dyn Any>,
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
        pub fn map_to_props(&self) -> Props {
            Props {}
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
    pub struct Props {}

    impl Render for Props {
        fn render(&self) -> Element {
            todo!()
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

#[derive(PartialEq)]
struct TodoAppProps {
    todos: Vec<Todo>,
    visibility_filter: VisibilityFilter,
    text_input: text_input::Props,
}

impl Render for TodoAppProps {
    fn render(&self) -> Element {
        let filtered_todos = self
            .todos
            .iter()
            .filter(|todo| match self.visibility_filter {
                VisibilityFilter::ShowAll => true,
                VisibilityFilter::ShowCompleted => todo.completed,
            })
            .cloned()
            .collect::<Vec<_>>();

        render![
            TodoListProps {
                todos: filtered_todos,
            },
            VisibilityFilterProps {
                visibility_filter: self.visibility_filter,
            },
            self.text_input.clone(),
        ]
    }
}
#[derive(PartialEq)]
struct TodoListProps {
    todos: Vec<Todo>,
}

impl Render for TodoListProps {
    fn render(&self) -> Element {
        let mut elements = vec![];

        for (index, todo) in self.todos.iter().enumerate() {
            elements.push(view(TodoProps {
                text: todo.text.clone(),
                completed: todo.completed,
                index,
            }));
        }

        div(elements)
    }
}

#[derive(PartialEq)]
struct TodoProps {
    text: String,
    completed: bool,
    index: usize,
}

impl Render for TodoProps {
    fn render(&self) -> Element {
        text(&self.text).event(|build| {
            let index = self.index;
            build.on_click_fn(move |_| Some(TodoEvent::ToggleTodo { index }));

            build.on_click(TodoEvent::ToggleTodo { index: self.index });
        })
    }
}

fn text(text: impl ToString) -> Element {
    todo!()
}

impl Element {
    fn event(self, build: impl FnOnce(&mut EventBuilder)) -> Element {
        todo!()
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

#[derive(PartialEq)]
struct VisibilityFilterProps {
    visibility_filter: VisibilityFilter,
}

impl Render for VisibilityFilterProps {
    fn render(&self) -> Element {
        todo!()
    }
}
