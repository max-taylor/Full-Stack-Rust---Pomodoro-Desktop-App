mod app;
mod components;
mod helpers;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
