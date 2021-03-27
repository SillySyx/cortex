use yew::prelude::*;

pub enum Messages {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: &'static str,
	#[prop_or_default]
	pub description: &'static str,
	#[prop_or_default]
    pub children: Children,
}

pub struct PageHeader {
    props: Props,
}

impl Component for PageHeader {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="page-header">
				{ self.render_children() }
                <h1 class="page-header-title">{ self.props.title }</h1>
				{ self.render_description() }
            </div>
        }
    }
}

impl PageHeader {
	fn render_description(&self) -> Html {
		if self.props.description.is_empty() {
			return html! {};
		}

		html! {
			<p class="page-header-description">{ self.props.description }</p>
		}
	}

	fn render_children(&self) -> Html {
		if self.props.children.is_empty() {
			return html! {};
		}

		html! {
			<div class="page-header-content">
				{ self.props.children.clone() }
			</div>
		}
	}
}