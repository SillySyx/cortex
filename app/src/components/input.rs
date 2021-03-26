use yew::{
    prelude::*, 
    web_sys::HtmlInputElement
};

pub enum Messages {
    ValueChanged(String),
    KeyPressed(KeyboardEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or(String::new())]
    pub value: String,
	#[prop_or(false)]
    pub focus: bool,
	#[prop_or(false)]
    pub password: bool,
	#[prop_or("")]
    pub class: &'static str,
	#[prop_or("")]
    pub placeholder: &'static str,
	#[prop_or("")]
    pub lable: &'static str,
	#[prop_or(String::new())]
    pub error: String,
	#[prop_or_default]
    pub children: Children,
	#[prop_or_default]
    pub value_changed: Callback<String>,
	#[prop_or_default]
    pub submitted: Callback<String>,
	#[prop_or_default]
    pub aborted: Callback<()>,
}

pub struct InputBox {
    props: Props,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    value: String,
}

impl Component for InputBox {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = props.value.clone();

        Self {
            props,
            link,
            node_ref: NodeRef::default(),
            value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ValueChanged(value) => {
                self.value = value.clone();
                self.props.value_changed.emit(value.clone());
            },
            Messages::KeyPressed(event) => {
                match event.key().as_str() {
                    "Enter" => self.props.submitted.emit(self.value.clone()),
                    "Escape" => self.props.aborted.emit(()),
                    _ => {},
                };
            }
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.value != props.value {
            self.props.value = props.value;
            return true;
        }
        if self.props.error != props.error {
            self.props.error = props.error;
            return true;
        }
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.focus {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        }
    }

    fn view(&self) -> Html {
        let input_type = match self.props.password {
            true => "password",
            false => "text",
        };

        html! {
            <div class=("input-box", self.props.class)>
                { self.render_lable() }

                <div class="input-box-container">
                    <input 
                        ref=self.node_ref.clone()
                        type=input_type 
                        value=self.props.value
                        placeholder=self.props.placeholder
                        oninput=self.link.callback(|e: InputData| Messages::ValueChanged(e.value))
                        onkeyup=self.link.callback(|e| Messages::KeyPressed(e)) />
                    
                    { self.props.children.clone() }
                </div>

                { self.render_error() }
            </div>
        }
    }
}

impl InputBox {
    fn render_lable(&self) -> Html {
        if self.props.lable.is_empty() {
            return html! {};
        }

        html! {
            <lable class="input-box-lable">{ self.props.lable }</lable>
        }
    }

    fn render_error(&self) -> Html {
        if self.props.error.is_empty() {
            return html! {};
        }

        html! {
            <div class="input-box-error">
                {self.props.error.clone()}
            </div>
        }
    }
}