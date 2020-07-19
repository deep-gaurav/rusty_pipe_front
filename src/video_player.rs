use super::downloader::{send_future, DownloaderExample};
use super::search_result::layout_result;
use rusty_pipe::youtube_extractor::error::ParsingError;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;
use rusty_pipe::youtube_extractor::stream_extractor::YTStreamExtractor;
use rusty_pipe::youtube_extractor::trending_extractor::YTTrendingExtractor;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct VideoPlayer {
    props: Props,
    link: ComponentLink<Self>,
    vid_ref: NodeRef,
    audio_ref: NodeRef,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub fullpage: bool,
    pub extractor: YTStreamExtractor<DownloaderExample>,
}

#[derive(Serialize)]
struct VideoSource {
    url: String,
    quality: String,
    mimeType: String,
    bitrate: i64,
    height: i64,
    contentLength: String,
}

#[derive(Serialize)]
struct AudioSource {
    url: String,
    bitrate: i64,
    quality: String,
    mimeType: String,
    contentLength: String,
}

use rusty_pipe::youtube_extractor::stream_extractor::StreamItem;
#[derive(Serialize)]
struct Source {
    videoOnlyStreams: Vec<StreamItem>,
    audioOnlyStreams: Vec<StreamItem>,
}

#[wasm_bindgen(
    inline_js = r##"export function setmedia(title, author,image,height,width) {
    console.log("setmedia",title)
    if ('mediaSession' in navigator) {
        navigator.mediaSession.metadata = new MediaMetadata({
          title: title,
          artist: author,
          album: '',
          artwork: [
              {
                  src: image, size:width+'x'+height, type: 'image/png'
              }
          ]
        });
    }
}"##
)]
extern "C" {
    fn setmedia(title: &str, author: &str, image: &str, height: i64, width: i64);
}

impl Component for VideoPlayer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            vid_ref: NodeRef::default(),
            audio_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        let mut thumbs = self
            .props
            .extractor
            .get_video_thumbnails()
            .unwrap_or_default();
        let pref_width = 720_f64;

        thumbs.sort_by_key(|t| (t.width as f64 - pref_width).abs() as u64);

        let thumburl = thumbs.first().map(|t| t.url.as_str()).unwrap_or_default();

        let video_streams = self
            .props
            .extractor
            .get_video_only_stream()
            .unwrap_or_default();
        let audio_streams = self.props.extractor.get_audio_streams().unwrap_or_default();
        let source = Source {
            videoOnlyStreams: video_streams,
            audioOnlyStreams: audio_streams,
        };
        let sourcejson = serde_json::to_string(&source).unwrap_or_default();

        let id = self.props.extractor.get_video_id();
        let name = self.props.extractor.get_name().unwrap_or_default();
        let author_name = self.props.extractor.get_uploader_name().unwrap_or_default();

        setmedia(
            &name,
            &author_name,
            thumburl,
            thumbs.first().map(|f| f.width).unwrap_or_default() as i64,
            thumbs.first().map(|f| f.height).unwrap_or_default() as i64,
        );
        html! {
            <figure class="image is-4by2"   >
                <bul-player data=sourcejson ref=self.vid_ref.clone() style="width:100%;display:block;" poster=thumburl id=id>
                </bul-player>
            </figure>
        }
    }
}
