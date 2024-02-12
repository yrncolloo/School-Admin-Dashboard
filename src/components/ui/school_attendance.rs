use stylist::{style, yew::styled_component};
use yew::{html, Html};
use crate::components::ui::{title::Title, title::TitleLevel};

#[styled_component]
pub fn SchoolAttendance() -> Html{
    let style = style!{
        border:1px solid black;
        .inner-box{
            border:1px solid black;
            margin:3%;
            width:100%;
            margin-top:9%;
        }
        display:flex;

    }.unwrap();
    html!{
        <div class={style}>
            <Title level={TitleLevel::Three}> {"School Attendance"} </Title>
            <div class={"inner-box"}>
                    <p> {"Student"}</p>
                    <p> {"503"}</p>
                    <p> {"33"}</p>
            </div>
            <div class={"inner-box"}>
                    <p> {"Teacher"}</p>
                    <p> {"150"}</p>
                    <p> {"2"}</p>
            </div>


        </div>

    }
    
}
