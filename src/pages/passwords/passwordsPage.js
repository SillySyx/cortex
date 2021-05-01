import { Component } from 'react';

import { ListView } from './listView';

export class PasswordsPage extends Component {
	constructor(props) {
		super(props);

		this.state = {
			view: "list",
		};
	}

	changeView(view) {
		this.setState({
			view: view,
		});
	}

	render() {
		return (
			<>
			{ this.state.view === "list" &&
				<ListView changeView={view => this.changeView(view)} />
			}
			</>
		);
	}
}