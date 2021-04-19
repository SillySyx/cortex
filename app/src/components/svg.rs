use yew::prelude::*;

pub enum Messages {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
	#[prop_or_default]
    pub class: String,
	#[prop_or_default]
    pub clicked: Callback<()>,
}

pub struct Svg {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Svg {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::Clicked => {
                self.props.clicked.emit(());
                false
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.src = props.src;
        self.props.class = props.class;
        true
    }

    fn view(&self) -> Html {
        let src = self.props.src.clone() + "#src";

        html! {
            <svg class={self.props.class.clone()} onclick={self.link.callback(|_| Messages::Clicked)}>
                <use href={src}></use>
            </svg>
        }
    }
}