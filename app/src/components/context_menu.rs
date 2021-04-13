use yew::prelude::*;

pub enum Messages {
    Open,
    Close,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
    pub children: Children,
	#[prop_or(false)]
    pub open: bool,
}

pub struct ContextMenu {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ContextMenu {
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
            Messages::Open => {
                self.props.open = true;
                true
            },
            Messages::Close => {
                self.props.open = false;
                true
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.open = props.open;
        self.props.children = props.children;
        false
    }

    fn view(&self) -> Html {
        let open = match self.props.open {
            true => " open",
            false => "",
        };

        let open_event = self.link.callback(|e: MouseEvent| {
            e.stop_propagation();
            Messages::Open
        });

        let close_event = self.link.callback(|e: MouseEvent| {
            e.stop_propagation();
            Messages::Close
        });

        html! {
            <div class=("context-menu", open) onclick=open_event>
                <div class="context-menu-backdrop animation-fade" onclick=close_event></div>
                { self.props.children.clone() }
            </div>
        }
    }
}