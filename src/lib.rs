use stylist::style;
use yew::prelude::*;
pub mod components;
use components::{sidebar::Sidebar, main_section::MainSection};

#[function_component]
pub fn App() -> Html{
    let style = style!{
            padding:1rem;
            margin-left:15rem;
            background-color:#f4f4f4;
            display:flex;
            margin-top:13px;
            margin-left:15.8rem;
            margin-right:13px;
            height:91vh;
            border-radius: 0px 15px 15px 0px;

    }.unwrap();
    html! {
        <>
        <div class={style}>
            <MainSection/>
        </div>
            <Sidebar />
        </>
    }

}

