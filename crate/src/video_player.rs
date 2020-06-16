use super::downloader::{send_future, DownloaderExample};
use super::search_result::layout_result;
use rusty_pipe::youtube_extractor::error::ParsingError;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;
use rusty_pipe::youtube_extractor::trending_extractor::YTTrendingExtractor;
use rusty_pipe::youtube_extractor::stream_extractor::YTStreamExtractor;
use yew::prelude::*;

pub struct VideoPlayer{
    props: Props,
    link: ComponentLink<Self>
}

pub enum Msg{

}

#[derive(Properties,Clone)]
pub struct Props{
    pub fullpage:bool,
    pub extractor:YTStreamExtractor<DownloaderExample>
}

impl Component for VideoPlayer{

    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        Self{
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props=_props;
        true
    }

    fn view(&self) -> Html {

        let mut thumbs = self.props.extractor.get_video_thumbnails().unwrap_or_default();
        let pref_width = 720_f64;
        
        thumbs.sort_by_key(|t|(t.width as f64 -pref_width).abs() as u64 );

        let thumburl = thumbs.first().map(|t|t.url.as_str()).unwrap_or_default();

        let video_streams = self.props.extractor.get_video_stream().unwrap_or_default();
        let video_sources = video_streams.iter().map(
            |vid|{
                let url = vid.url.clone().unwrap_or_default();
                let mimetype = vid.mimeType.clone();
                html!{
                    <source src=url type=mimetype />
                }
            }
        );

        html!{
            <figure class="image is-4by2" style={

                let mut styles = String::new();
                if !self.props.fullpage{
                    styles = format!("position:fixed; bottom:5px; right:5px; width: 240px; z-index:20")
                }
                styles

            } >
                <video controls=true >
                    {
                        for video_sources
                    }
                </video>
            </figure>
        }
    }

}