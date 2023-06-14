use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
pub struct TodoList {
    todos: Vec<String>,
    completed_todos: Vec<String>,
    todo: String,
}
pub enum Msg {
    UpdateTodo(String),
    AddTodoList,
    CompleteTodo(usize),
}

impl Component for TodoList {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            todos: vec![],
            completed_todos: vec![],
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
            Msg::CompleteTodo(idx) => {
                let todo = self.todos.remove(idx);
                self.completed_todos.push(todo);
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
                <h2>{"タスク一覧"}</h2>
                <h3>{"完"}</h3>
                <ul>
                    {for self.completed_todos.iter().map(|todo| {
                        html! {
                            <li>
                            {todo}
                            </li>

                        }
                    })}
                </ul>
                <h3>{"未完"}</h3>
                <ul>
                    {for self.todos.iter().enumerate().map(|(idx,todo)| {
                        html! {
                            <li>
                            {todo}
                            <button onclick={ctx.link().callback(move |_| {
                                Msg::CompleteTodo(idx)
                            })}>{"完了する"}</button>
                            </li>

                        }
                    })}
                </ul>
            </div>
        }
    }
}
