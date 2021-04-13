use yew::{
    prelude::*, 
    web_sys::HtmlInputElement
};

pub enum Messages {
    ValueChanged(String),
    KeyPressed(KeyboardEvent),
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
	#[prop_or_default]
    pub value: String,
	#[prop_or_default]
    pub focus: bool,
	#[prop_or_default]
    pub mandatory: bool,
	#[prop_or_default]
    pub password: bool,
	#[prop_or_default]
    pub class: String,
	#[prop_or_default]
    pub placeholder: String,
	#[prop_or_default]
    pub label: String,
	#[prop_or_default]
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
}

impl Component for InputBox {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ValueChanged(value) => {
                self.props.value = value.clone();
                self.props.value_changed.emit(value.clone());
            },
            Messages::KeyPressed(event) => {
                match event.key().as_str() {
                    "Enter" => self.props.submitted.emit(self.props.value.clone()),
                    "Escape" => self.props.aborted.emit(()),
                    _ => {},
                };
            }
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.value = props.value;
        self.props.error = props.error;
        true
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
            <div class=("input-box", &self.props.class)>
                { self.render_label() }

                <div class="input-box-container">
                    <input 
                        ref=self.node_ref.clone()
                        type=input_type 
                        value=&self.props.value
                        placeholder=&self.props.placeholder
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
    fn render_label(&self) -> Html {
        if self.props.label.is_empty() {
            return html! {};
        }

        let show_error_indicator = (self.props.mandatory && self.props.value.is_empty()) || !self.props.error.is_empty();

        html! {
            <div class="input-box-label">
                <label>{ &self.props.label }</label>
                { match show_error_indicator {
                    false => html! {},
                    true => html! {
                        <span class="input-box-error-indicator">{"*"}</span>
                    },
                }}
            </div>
        }
    }

    fn render_error(&self) -> Html {
        if self.props.error.is_empty() {
            return html! {};
        }

        html! {
            <div class="input-box-error">
                { &self.props.error }
            </div>
        }
    }
}