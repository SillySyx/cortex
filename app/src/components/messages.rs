use yew::prelude::*;

pub enum Messages {
    UnlockApp(&'static str),
    ChangePage(&'static str),
    MainPageChangeView(&'static str),
    LoginPageChangeView(&'static str),
    LoginPageKeyPressed(KeyboardEvent),
}