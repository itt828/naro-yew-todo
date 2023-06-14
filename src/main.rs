pub mod components;
use crate::components::todolist::TodoList;
use yew::prelude::*;

struct App {}

enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
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
