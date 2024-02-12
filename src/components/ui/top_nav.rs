use stylist::{style, yew::styled_component};
use yew::{html, Html};
use crate::components::ui::title::{Title, TitleLevel};


#[styled_component]
pub fn TopNav() -> Html{

    let style = style!{
        .items{
        text-align:center;
        margin:0.3rem;
        padding:0.5rem;
        width:100%;
        background-color:#E7E7E7;
        border-radius:25px;
        }
        a{
            text-decoration:none;
        }
        .items:hover{
            background-color:#DEDEDE;
            cursor:pointer;
        }
        display:flex;
        width:100vh;
        align-item:left;

    }.unwrap();
    
    let staff = vec!["Student", "Teachers", "Working stuff"];
    let total_num = vec!["150", "150", "150"];
    html!{

        <div class={style}>
            {list_items({staff}, {total_num})}
        </div>
    }
}


fn list_items(list:Vec<&str>, taskss:Vec<&str>) -> Vec<Html>{
    list.into_iter().zip(taskss).map(|(staff, total)| html!{
        <div class={"items"}>
                <p> {staff}</p>
                <Title level={TitleLevel::Three} > {total}</Title>
        </div>

    }).collect()

}
