use yew::prelude::*;
pub mod todolist;

pub struct TodoList {}
pub enum Msg {}

impl Component for TodoList {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                    <h1>{"Hello World!"}</h1>
            </div>
        }
    }
}
