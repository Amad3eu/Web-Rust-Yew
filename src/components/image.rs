use yew::{html, Html};

use crate::Model;
// consider test with Self and Component method later?
pub fn view_image(value: &str) -> Html<Model> {
    html! {
        <img class=("flex", "margin-top-one-and-a-half", "max-width-half"), src=value, ></img>
    }
}
