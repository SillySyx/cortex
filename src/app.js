import { Component } from 'react';
import { Route, Switch, BrowserRouter } from 'react-router-dom';

import { HomePage } from './pages/home/page';
import { KnowledgePage } from './pages/knowledge/page';
import { LoginPage } from './pages/login/page';
import { PasswordsPage } from './pages/passwords/page';
import { SyncPage } from './pages/sync/page';

export class App extends Component {
	render() {
		if (false) {
			return (
				<LoginPage />
			);
		}

		return (
			<BrowserRouter>
				<Switch>
					<Route path="/passwords" component={PasswordsPage} />
					<Route path="/knowledge" component={KnowledgePage} />
					<Route path="/sync" component={SyncPage} />
					<Route component={HomePage} />
				</Switch>
			</BrowserRouter>
		);
	}
}