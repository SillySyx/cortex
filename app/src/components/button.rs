use yew::prelude::*;

pub enum Messages {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
    pub active: bool,
	#[prop_or_default]
    pub disabled: bool,
	#[prop_or_default]
    pub class: String,
	#[prop_or_default]
    pub children: Children,
	#[prop_or_default]
    pub clicked: Callback<()>,
}

pub struct Button {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Button {
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
                if self.props.disabled {
                    return false;
                }

                self.props.clicked.emit(());
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.active != props.active {
            self.props.active = props.active;
            return true;
        }
        if self.props.disabled != props.disabled {
            self.props.disabled = props.disabled;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        let active = match self.props.active {
            true => "active",
            false => "",
        };
        let disabled = match self.props.disabled {
            true => "disabled",
            false => "",
        };

        let clicked = self.link.callback(|e: MouseEvent| {
            e.stop_propagation();
            Messages::Clicked
        });

        html! {
            <div class=("main-button", "animation-grow", &self.props.class, active, disabled) onclick=clicked>
                { self.props.children.clone() }
            </div>
        }
    }
}