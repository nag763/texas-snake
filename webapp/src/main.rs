use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <script type="module">
            {"
import init from '/games/snake.js'
    console.log('Hello');
  init()
  "}

            </script>
            {"Hello world!"}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
