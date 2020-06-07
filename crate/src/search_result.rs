use yew::{ComponentLink, Component, Html};
use rusty_pipe::youtube_extractor::search_extractor::YTSearchExtractor;
use yew::prelude::*;

pub struct SearchResult{
    link:ComponentLink<Self>,
    next_page_extractors:Vec<YTSearchExtractor>,
    is_loading:bool,
    last_reached:bool,
    props:Props
}

#[derive(Clone,Properties,PartialEq)]
pub struct Props{
    pub extractor: YTSearchExtractor,
    pub query: String
}

pub enum Msg{
    LoadNext,
    Loaded(YTSearchExtractor),
    LoadFail,
}

impl Component for SearchResult{
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            props,
            next_page_extractors:vec![],
            is_loading:false,
            last_reached:false
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg{
            Msg::LoadFail=>{
                self.is_loading=false;
                self.last_reached=true;
                true
            }
            Msg::Loaded(extractor)=>{
                self.next_page_extractors.push(extractor);
                self.is_loading=false;
                true
            }
            Msg::LoadNext=>{
                if self.is_loading{
                    false
                }else{
                    self.is_loading=true;
                    let extractor = self.next_page_extractors.last().unwrap_or(&self.props.extractor);
                    let next_page_url = extractor.get_next_page_url();
                    match next_page_url{
                        Ok(next_page_url)=>{
                            match next_page_url{
                                Some(url)=>{
                                    let query = self.props.query.clone();
                                    use super::downloader::{send_future,DownloaderExample};
                                    let future = async move {
                                        let ytex = YTSearchExtractor::new(DownloaderExample,&query,Some(url)).await;
                                        let ytex = ytex.ok();
                                        if let Some(ytex)=ytex{
                                          Msg::Loaded(ytex)
                                        }else{
                                          Msg::LoadFail
                                        }
                                    };
                                    send_future(self.link.clone(),future);
                                }
                                None=>{
                                    self.last_reached=true;
                                }
                            }
                        }
                        Err(err)=>{
                            log::error!("{:#?}",err);
                            self.last_reached=true;
                        }
                    }
                    true
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let cardwidth = 320_f64;
        let mut results = self.props.extractor.search_results();
        match results{
            Ok(mut results)=>{

                for page in self.next_page_extractors.iter(){
                    if let Ok(mut page_result)=page.search_results(){
                        results.append(&mut page_result);
                    }
                }

                let window_width = yew::utils::window().inner_width().expect("Cant get window width").as_f64().expect("window width not number");

                let cardscolumn = (window_width/cardwidth).floor() as usize;

                let mut rows= vec![];

                

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

                            if let Some(thumb)= uploader_thumbnails.iter().next(){
                                avatar = String::from(&thumb.url);
                            }

                            html!{
                                <div class="tile is-child " style="padding:10px">
                                <div class="card" style="height:100%">
                                    <div class="card-image">
                                        <figure class="image is-4by2">
                                        <img src=thumburl alt="Placeholder image"/>

                                        <span class="tag" style="position:absolute;right:5px;bottom:5px;">
                                            {duration}
                                        </span>
                                        </figure>
                                    </div>
                                    <div class="card-content">
                                        <div class="media">
                                        <div class="media-left">
                                            <figure class="image is-48x48">
                                            <img src=avatar alt="Placeholder image"/>
                                            </figure>
                                        </div>
                                        <div class="media-content">
                                            <p class="title is-6">{vid_name}</p>
                                            <p class="subtitle is-6">{author_name}</p>
                                        </div>
                                        </div>

                                        <div class="content">
                                        
                                        </div>
                                    </div>
                                    </div>
                                    </div>
                            }
                        }
                        YTSearchItem::PlaylistInfoItem(playlistinfo)=>{
                            html!{

                            }
                        }
                        YTSearchItem::ChannelInfoItem(channelinfo)=>{
                            html!{

                            }
                        }
                    }
                });

                for i in 0..(m.len()/cardscolumn){
                    let mut row = vec![];
                    for j in 0..cardscolumn{
                        if let Some(item)= m.next(){
                            row.push(item.clone());
                        }
                    }
                    // let row = row.iter().map(|c|html!{<>{c}</>});
                    rows.push(
                        html!{

                            <div class="tile is-ancestor">
                                <div class="tile is-parent">
                                    {for row}
                                </div>
                            </div>
                        }
                    );
                }
                

                html!{
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
            Err(err)=>{
                html!{
                    <article class="message is-danger">
                        <div class="message-header">
                            <p>{"Error"}</p>
                            <button class="delete" aria-label="delete"></button>
                        </div>
                        <div class="message-body">
                           {
                               format!("{:#?}",err)
                           }
                        </div>
                    </article>
                }
            }
        }
    }
}