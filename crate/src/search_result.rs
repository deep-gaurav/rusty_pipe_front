use super::downloader::{send_future, DownloaderExample};
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;
use yew::prelude::*;

use yew::{Component, ComponentLink, Html};
use super::route_comp::RouteComponent;

pub struct SearchResult {
    link: ComponentLink<Self>,
    next_page_extractors: Vec<YTSearchExtractor>,
    is_loading: bool,
    last_reached: bool,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    // pub extractor: YTSearchExtractor,
    pub query: String,
}

pub enum Msg {
    LoadNext,
    Loaded(YTSearchExtractor),
    LoadFail,
}

pub fn layout_result(cardwidth: f64, results: &Vec<YTSearchItem>) -> Vec<Html> {

    use super::app::{AppRoute,go_to_route};
    {
        let window_width = yew::utils::window()
            .inner_width()
            .expect("Cant get window width")
            .as_f64()
            .expect("window width not number");

        log::info!("window_width: {}",window_width);
        let cardscolumn = {
            let t = ((window_width / cardwidth).floor() as usize);
            if t==0{
                1
            }else{
                t
            }
        };
        log::info!("cards_column: {}",cardscolumn);

        let mut rows = vec![];

        let mut m=results.iter().map(|result|{
                use rusty_pipe::youtube_extractor::search_extractor::YTSearchItem;

                match result{
                    YTSearchItem::StreamInfoItem(streaminfo)=>{
                        
                        let vid_name = streaminfo.get_name().unwrap_or("".to_owned());
                        let author_name = streaminfo.get_uploader_name().unwrap_or("author".to_owned());

                        let mut thumbnails = streaminfo.get_thumbnails().unwrap_or(vec![]);
                        thumbnails.sort_by_key(|t| (cardwidth - t.width as f64).abs() as u64);
                        let mut thumburl=String::default();
                        if let Some(thumb)= thumbnails.iter().next(){
                            thumburl = String::from(&thumb.url);
                        }

                        let uploader_thumbnails = streaminfo.get_uploader_thumbnails().unwrap_or(vec![]);
                        let mut avatar=String::default();

                        let duration = streaminfo.get_textual_duration().unwrap_or_default();

                        let viewcount = streaminfo.get_textual_view_count().unwrap_or_default();
                        let upload_date = streaminfo.get_textual_upload_date().unwrap_or_default();

                        if let Some(thumb)= uploader_thumbnails.iter().next(){
                            avatar = String::from(&thumb.url);
                        }

                        let video_id = streaminfo.video_id().unwrap_or_default();

                        let approute = yew_router::route::Route::from(super::app::AppRoute::Video(video_id));

                        html!{
                            <div class="tile is-child " style="padding:10px">
                            <RouteComponent route=approute>
                            <div class="card" style="height:100%">
                                <div class="card-image">
                                    <figure class="image is-4by2">
                                    <img src=thumburl alt="Video Thumbnail"/>

                                    <span class="tag" style="position:absolute;right:5px;bottom:5px;">
                                        {duration}
                                    </span>
                                    </figure>
                                </div>
                                <div class="card-content">
                                    <div class="media">
                                    <div class="media-left">
                                        <figure class="image is-48x48">
                                        <img src=avatar alt="Channelavatar" style="border-radius: 50%"/>
                                        </figure>
                                    </div>
                                    <div class="media-content">
                                        <p class="title is-6">{vid_name}</p>
                                        <p class="subtitle is-6">{author_name}</p>
                                    </div>
                                    </div>
                                    <div class="content">
                                        <div class="level is-mobile">
                                            <div class="level-item">
                                                {viewcount}
                                            </div>
                                            <div class="level-item">
                                                {upload_date}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                </div>
                                </RouteComponent>
                                </div>
                        }
                    }
                    YTSearchItem::PlaylistInfoItem(playlistinfo)=>{

                        let name = playlistinfo.get_name().unwrap_or("".to_owned());

                        let mut thumbnails = playlistinfo.get_thumbnails().unwrap_or(vec![]);
                        thumbnails.sort_by_key(|t| (cardwidth - t.width as f64).abs() as u64);
                        let mut thumburl=String::default();
                        if let Some(thumb)= thumbnails.iter().next(){
                            thumburl = String::from(&thumb.url);
                        }

                        let uploader = playlistinfo.get_uploader_name().unwrap_or_default();

                        let video_count = playlistinfo.get_stream_count().unwrap_or_default();

                        html!{
                            <div class="tile is-child " style="padding:10px">
                            <div class="card" style="height:100%">
                                <div class="card-image">
                                    <figure class="image is-4by2">
                                    <img src=thumburl alt="Playlist Thumbnail"/>
                                    <div class="container level is-mobile" style="position:absolute;right:0px;bottom:0px;width:50%;height:100%;background-color:#808080c3;">
                                        <div style="width:100%" class="has-text-centered has-text-light">
                                            <span class="icon is-large">
                                                <i class="fas fa-2x fa-play"></i>
                                            </span>
                                            <p class="is-5">{format!("{} videos",video_count)}</p>
                                        </div>
                                    </div>
                                    </figure>
                                </div>
                                <div class="card-content">
                                    <div class="media">
                                    <div class="media-content">
                                        <p class="title is-6">{name}</p>
                                        <p class="subtitle is-6">{uploader}</p>
                                    </div>
                                    </div>
                                </div>
                                </div>
                                </div>
                        }
                    }
                    YTSearchItem::ChannelInfoItem(channelinfo)=>{
                        let mut avatars = channelinfo.get_thumbnails().unwrap_or(vec![]);
                        avatars.sort_by_key(|t| (cardwidth - t.width as f64).abs() as u64);
                        let mut thumburl=String::default();
                        if let Some(thumb)= avatars.iter().next(){
                            thumburl = String::from(&thumb.url);
                        }
                        let name = channelinfo.get_name().unwrap_or_default();
                        thumburl = rusty_pipe::utils::utils::fix_thumbnail_url(&thumburl);

                        let subscribers = channelinfo.get_subscriber_count().map(|i|format!("{} subs",i)).unwrap_or_default();
                        let videos = channelinfo.get_stream_count().map(|i|format!("{} videos",i)).unwrap_or_default();
                        html!{
                            <div class="tile is-child" style="padding:10px">
                                <div class="container mb-4 mt-1">
                                    <figure class="image is-1by1">
                                        <img src=thumburl alt="Channel Thumbnail" style="border-radius: 50%" />
                                    </figure>
                                    <p class="title is-6 has-text-centered mt-3 mb-1">{name}</p>
                                    <div class="level is-mobile">
                                        <div class="level-item">
                                            {subscribers}
                                        </div>
                                        <div class="level-item">
                                            {videos}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                }
            });

        for _i in 0..(m.len() / cardscolumn) {
            let mut row = vec![];
            for _j in 0..cardscolumn {
                if let Some(item) = m.next() {
                    row.push(item.clone());
                }
            }
            // let row = row.iter().map(|c|html!{<>{c}</>});
            rows.push(html! {

                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        {for row}
                    </div>
                </div>
            });
        }

        rows
    }
}

impl Component for SearchResult {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let query = props.query.clone();
        let future = async move {
            let ytex = YTSearchExtractor::new(DownloaderExample, &query, None).await;
            let ytex = ytex.ok();
            if let Some(ytex) = ytex {
                Msg::Loaded(ytex)
            } else {
                Msg::LoadFail
            }
        };
        send_future(link.clone(), future);
        Self {
            link,
            props,
            next_page_extractors: vec![],
            is_loading: true,
            last_reached: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadFail => {
                self.is_loading = false;
                self.last_reached = true;
                true
            }
            Msg::Loaded(extractor) => {
                self.next_page_extractors.push(extractor);
                self.is_loading = false;
                true
            }
            Msg::LoadNext => {
                if self.is_loading {
                    false
                } else {
                    self.is_loading = true;
                    let extractor = self.next_page_extractors.last().unwrap();
                    let next_page_url = extractor.get_next_page_url();
                    match next_page_url {
                        Ok(next_page_url) => match next_page_url {
                            Some(url) => {
                                let query = self.props.query.clone();
                                use super::downloader::{send_future, DownloaderExample};
                                let future = async move {
                                    let ytex = YTSearchExtractor::new(
                                        DownloaderExample,
                                        &query,
                                        Some(url),
                                    )
                                    .await;
                                    let ytex = ytex.ok();
                                    if let Some(ytex) = ytex {
                                        Msg::Loaded(ytex)
                                    } else {
                                        Msg::LoadFail
                                    }
                                };
                                send_future(self.link.clone(), future);
                            }
                            None => {
                                self.last_reached = true;
                            }
                        },
                        Err(err) => {
                            log::error!("{:#?}", err);
                            self.last_reached = true;
                        }
                    }
                    true
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        log::info!("change : {:#?}", _props.query);
        if self.props.query != _props.query {
            self.is_loading = true;
            self.next_page_extractors.clear();
            self.props = _props.clone();

            let query = _props.query.clone();
            let future = async move {
                let ytex = YTSearchExtractor::new(DownloaderExample, &query, None).await;
                let ytex = ytex.ok();
                if let Some(ytex) = ytex {
                    Msg::Loaded(ytex)
                } else {
                    Msg::LoadFail
                }
            };
            send_future(self.link.clone(), future);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let cardwidth = 320_f64;
        let mut results = vec![];

        for page in self.next_page_extractors.iter() {
            if let Ok(mut page_result) = page.search_results() {
                results.append(&mut page_result);
            }
        }
        let rows = layout_result(cardwidth, &results);

        html! {
            <>
            {for rows}
            {
                if !self.last_reached{
                    html!{
                        <button onclick=self.link.callback(|_|Msg::LoadNext) class={
                            let mut classes = "button is-fullwidth".to_string();
                            if self.is_loading {
                              classes = format!("{} is-loading",classes);
                            }
                            classes
                        }>{"Load More"}</button>
                    }
                }else{
                    html!{}
                }
            }
            </>
        }
    }
}
