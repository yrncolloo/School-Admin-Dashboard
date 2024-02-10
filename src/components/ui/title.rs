use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Html,
    pub level:TitleLevel

}

#[derive(PartialEq)]
pub enum TitleLevel{
    One,
    Two,
    Three,
    Four,
    Five,
    Six
}

impl TitleLevel {
    
    pub fn render(&self, title:Html) -> Html{
        match self {
            Self::One => html!{<h1> {title}</h1>},
            Self::Two => html!{<h2> {title}</h2>},
            Self::Three => html!{<h3> {title}</h3>},
            Self::Four => html!{<h4> {title}</h4>},
            Self::Five => html!{<h5> {title}</h5>},
            Self::Six => html!{<h6> {title}</h6>}
            
        }
        
    }
}

#[function_component]
pub fn Title(props:&Props) -> Html{
    html!{

        {props.level.render(props.children.clone())}

        }
}
