use yew::prelude::*;

pub enum Messages {
    ChangePage(&'static str),
    MainPageChangeView(&'static str),
    LoginPageChangeView(&'static str),
    LoginPageKeyPressed(KeyboardEvent),
}