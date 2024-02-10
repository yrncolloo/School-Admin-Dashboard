use std::string;

use stylist::{style, yew::styled_component};
use yew::{html, Html};

#[styled_component]
pub fn Sidebar() -> Html{
    let style = style!{
        width:15rem;
        height:96vh;
        position:fixed;
        background-color:#FF8C00;
        top:0;
        margin:13px;
        border-radius:15px 0px 0px 15px;
        .navbar-nav{
            list-style-type: none;
            padding:0;
            margin:0;
            display:flex;
            flex-direction:column;
            align-items:center;
        }
        .nav-item:nth-last-child(2){
            margin-top:auto;
        }
        .nav-link{
            display: flex;
            align-items: center;
            justify-content: center;
            height: 2.3rem;
            width: 13rem;
            margin: 0.5rem;
            border-radius: 5px;
        }
        .nav-link:hover{
            background-color:#fff;
            color:#FF8C00;
        }
        .container{
            position:relative;
            display:inline;
            align-items:center;
        }
        img{
            position:absolute;
            top:50%;
            right:10rem;
            transform:translateY(-50%);
            pointer-events:none;
        }
        a{
            text-decoration: none;
        }

    }.unwrap();

    let list = vec!["Dashboard", "Student", "Calendar", "Email Inbox", "Classes", "Teacher", "Staff", "support", "update"];

    html! {
        <div class={style}>
                <div class={"container"}>
                {list_items(list)}
                // <img src="images/shapes-solid.svg" alt="plus"/>
                </div>
        </div>
    }
    
}

fn list_items(list:Vec<&str>) -> Vec<Html>{
    list.into_iter().map(|item| html!{<ul class={"navbar-nav"}><li class={"nav-item"}> <a href="#" class={"nav-link"}>{item}</a> </li></ul>}).collect()
    
}
