pub mod components;
use crate::components::todolist::TodoList;
use yew::prelude::*;

struct App {
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <TodoList></TodoList>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
