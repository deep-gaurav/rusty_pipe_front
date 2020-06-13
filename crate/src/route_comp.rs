use yew::prelude::{*};
use yew_router::route::Route;
use super::app::{AppRoute,go_to_route};

pub struct RouteComponent{
    props:Props,
    link:ComponentLink<Self>
}

#[derive(Properties,Clone)]
pub struct Props{
    #[prop_or_default]
    pub children:Children,
    pub route:Route
}

pub enum Msg{
    Clicked
}

impl Component for RouteComponent{

    type Message=Msg;
    type Properties=Props;
    

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        Self{
            props,
            link
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked=>{

                go_to_route(self.props.route.clone());
                true
            }
        }
    }

    fn view(&self) -> Html {
        html!{
            <div onclick=self.link.callback(|_|Msg::Clicked)>
            {
                self.props.children.render()
            }
            </div>
        }
    }

}