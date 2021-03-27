use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
    pub icon: String,
	#[prop_or_default]
    pub title: String,
	#[prop_or_default]
    pub text: String,
	#[prop_or_default]
    pub children: Children,
}

pub struct Error {
    props: Props,
}

impl Component for Error {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.title != props.title {
            self.props.title = props.title;
            return true;
        }
        if self.props.text != props.text {
            self.props.text = props.text;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="error-message">
                { match self.props.icon.is_empty() {
                    true => html! {},
                    false => html! {
                        <img class="error-message-icon" src={&self.props.icon} alt="" />
                    },
                }}
                { match self.props.title.is_empty() {
                    true => html! {},
                    false => html! {
                        <h2 class="error-message-title">{&self.props.title}</h2>
                    },
                }}
                { match self.props.text.is_empty() {
                    true => html! {},
                    false => html! {
                        <p class="error-message-text">{&self.props.text}</p>
                    },
                }}
                { match self.props.children.is_empty() {
                    true => html! {},
                    false => html! {
                        <div class="error-message-content">
                            {self.props.children.clone()}
                        </div>
                    },
                }}
            </div>
        }
    }
}