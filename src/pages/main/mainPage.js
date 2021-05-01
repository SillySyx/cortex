import { Component } from 'react';

import { Button } from '../../components/button';

import { Logo } from '../../icons/logo';
import { Passwords } from '../../icons/passwords';
import { Knowledge } from '../../icons/knowledge';

import { HomePage } from '../../pages/home/homePage';
import { KnowledgePage } from '../../pages/knowledge/knowledgePage';
import { LoginPage } from '../../pages/login/loginPage';
import { PasswordsPage } from '../../pages/passwords/passwordsPage';
import { SyncPage } from '../../pages/sync/syncPage';

import { loadKey } from '../../services/crypto';

import './main.css';

export class MainPage extends Component {
	constructor(props) {
		super(props);

		this.state = {
			view: "home",
		};
	}

	render() {
		const [loggedIn, ] = loadKey();
		if (!loggedIn) {
			return (
				<LoginPage changeView={view => this.setState({ view: view })} />
			);
		}

		return (
            <div className="main-layout main-background">
                <aside className="main-menu">
                    <div className="menu-items">
                        <Logo class="main-menu-logo" clicked={() => this.setState({ view: "home"})} />

                        <Button active={this.state.view === "passwords"} clicked={() => this.setState({ view: "passwords"})}>
                            <Passwords class="main-button-icon" />
                        </Button>
                        <Button active={this.state.view === "knowledge"} clicked={() => this.setState({ view: "knowledge"})}>
                            <Knowledge class="main-button-icon" />
                        </Button>
                    </div>
                </aside>
                <section className="main-content">
					{ this.state.view === "passwords" &&
						<PasswordsPage changeView={view => this.setState({ view: view })} />
					}
					{ this.state.view === "knowledge" &&
						<KnowledgePage changeView={view => this.setState({ view: view })} />
					}
					{ this.state.view === "sync" &&
						<SyncPage changeView={view => this.setState({ view: view })} />
					}
					{ this.state.view === "home" &&
						<HomePage changeView={view => this.setState({ view: view })} />
					}
                </section>
            </div>
		);
	}
}