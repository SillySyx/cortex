use std::time::Duration;

use yew::prelude::*;
use yew::services::{TimeoutService, Task};

use super::{Button, PageHeader, InputBox, Error, Svg};
use crate::services::ClipboardService;

#[derive(PartialEq)]
enum Mode {
    New,
    Edit,
}

pub enum Messages {
    UpdateName(String),
    UpdateDescription(String),
    UpdatePassword(String),

    CopyDescription,
    CopyPassword,

    BackClicked,
    SaveClicked,
    RemoveClicked,
}


pub struct PasswordEditor {
    props: Props,
    link: ComponentLink<Self>,
    id: String,
    name: String,
    name_error: String,
    description: String,
    password: String,
    password_error: String,
    mode: Mode,
    timeout_task: Option<Box<dyn Task>>,
}

impl Component for PasswordEditor {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mode = match props.new_mode {
            true => Mode::New,
            false => Mode::Edit,
        };

        let id = props.id.clone();
        let name = props.name.clone();
        let description = props.description.clone();
        let password = props.password.clone();

        Self {
            props,
            link,
            id,
            name,
            name_error: String::from(""),
            description,
            password,
            password_error: String::from(""),
            mode,
            timeout_task: None,
        }
    }
}