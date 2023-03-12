mod app;
mod config;
mod egg;
mod env;
mod frog;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
