
use crate::components::ui::{title::Title, title::TitleLevel, top_nav::TopNav};
use stylist::yew::styled_component;
use yew::{html, Html};


#[styled_component]
pub fn MainSection() -> Html{
    
    html!{
        <div>
            <Title level={TitleLevel::One}> {"Admin Dashboard"}</Title>
            <p> {"Hello Shilla, welcome to your Dashboard"}</p>
            <TopNav/>

        </div>
    }
}
