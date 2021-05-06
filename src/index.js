import { Component, StrictMode } from 'react';
import { render } from 'react-dom';

import { LoginPage } from './pages/login/loginPage';
import { PasswordsPage } from './pages/passwords/passwordsPage';

import { loadKey } from './services/crypto';

class App extends Component {
	constructor(props) {
		super(props);

		this.state = {
			loggedIn: false,
		};
	}

	componentDidMount() {
		const [loggedIn, ] = loadKey();

		this.setState({
			loggedIn: loggedIn,
		});
	}

	render() {
		if (!this.state.loggedIn) {
			return (
				<LoginPage login={() => this.setState({ loggedIn: true })} />
			);
		}

		return (
			<PasswordsPage logout={() => this.setState({ loggedIn: false })} />
		);
	}
}

render(
	<StrictMode>
		<App />
	</StrictMode>,
	document.getElementById("root")
);