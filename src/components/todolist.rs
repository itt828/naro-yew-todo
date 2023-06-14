use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
pub struct TodoList {
    todos: Vec<String>,
    todo: String,
}
pub enum Msg {
    UpdateTodo(String),
    AddTodoList,
}

impl Component for TodoList {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            todos: vec![],
            todo: String::new(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTodo(todo) => {
                self.todo = todo;
                true
            }
            Msg::AddTodoList => {
                self.todos.push(self.todo.clone());
                self.todo = String::new();
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"ToDoリスト"}</h1>
                <input value={self.todo.clone()} oninput={ctx.link().callback(move |e: InputEvent| {
                    let ev: Event = e.dyn_into().unwrap();
                    let et = ev.target().unwrap();
                    let t: HtmlInputElement = et.dyn_into().unwrap();
                    Msg::UpdateTodo(t.value())
                })}/>
                <button onclick={ctx.link().callback(move |_| {
                    Msg::AddTodoList
                })}>{"登録"}</button>
                <ul>
                    {for self.todos.iter().map(|todo| {
                        html! {
                            <li>{todo}</li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}
