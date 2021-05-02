import { Component } from 'react';

import { ListView } from './listView';
import { AddCategoryView } from './addCategoryView';
import { EditCategoryView } from './editCategoryView';
import { AddPasswordView } from './addPasswordView';
import { EditPasswordView } from './editPasswordView';

import './passwords.css';

export class PasswordsPage extends Component {
	constructor(props) {
		super(props);

		this.state = {
			view: "list",
		};
	}

	changeView(view, id) {
		this.setState({
			view: view,
			id: id,
		});
	}

	render() {
		return (
			<>
			{ this.state.view === "list" &&
				<ListView changeView={(view, id) => this.changeView(view, id)} />
			}
			{ this.state.view === "add_category" &&
				<AddCategoryView changeView={(view, id) => this.changeView(view, id)} />
			}
			{ this.state.view === "edit_category" &&
				<EditCategoryView changeView={(view, id) => this.changeView(view, id)} categoryId={this.state.id} />
			}
			{ this.state.view === "add_password" &&
				<AddPasswordView changeView={(view, id) => this.changeView(view, id)} categoryId={this.state.id} />
			}
			{ this.state.view === "edit_password" &&
				<EditPasswordView changeView={(view, id) => this.changeView(view, id)} passwordId={this.state.id} />
			}
			</>
		);
	}
}