use crate::html_labels::Video;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos
        .into_iter()
        .map(|video| html! {<p>{ format!("{}: {}", video.speaker, video.title) }</p>})
        .collect()
}
