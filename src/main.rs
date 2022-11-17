mod html_labels;

use html_labels::{get_vedio_lists, Video};
use yew::prelude::*;
fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    let viedo_list = get_vedio_lists();

    let videos: Vec<Html> = viedo_list
        .into_iter()
        .map(|video| html! {<p>{ format!("{}: {}", video.speaker, video.title) }</p>})
        .collect();

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <h4>{"Group1"}</h4>
                <p>{ "John Doe: Building and breaking things" }</p>
                <p>{ "Jane Smith: The development process" }</p>
                <p>{ "Matt Miller: The Web 7.0" }</p>
                <p>{ "Tom Jerry: Mouseless development" }</p>

                <h4>{"Group2"}</h4>
                {videos}
            </div>
            <div>
                <h3>{ "John Doe: Building and breaking things" }</h3>
                <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            </div>
        </>
    }
}

// #[function_component(VideoList)]
// fn video_list() -> Html {

// }
