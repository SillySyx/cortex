import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { ContextMenu, ContextMenuContent } from '../../components/context-menu';
import { Button } from '../../components/button';

import { Cog } from '../../icons/cog';
import { Home } from '../../icons/home';
import { Passwords } from '../../icons/passwords';
import { Knowledge } from '../../icons/knowledge';

import './home.css';

export class HomePage extends Component {
	changeView(view) {
		if (this.props.changeView) {
			this.props.changeView(view);
		}
	}

	logout() {
		sessionStorage.removeItem("key");
		this.changeView("login");
	}

	render() {
		return (
			<div className="home-page">
				<PageHeader title="Welcome">
					<ContextMenu>
						<Cog class="page-header-icon" />
						<ContextMenuContent>
							<Button clicked={() => this.changeView("sync")}>Sync data</Button>
							<Button clicked={() => this.logout()}>Logout</Button>
						</ContextMenuContent>
					</ContextMenu>
				</PageHeader>

				<div className="home-page-content">
					<Home class="home-page-icon" />
					<div className="quick-links">
						<div className="quick-link" onClick={() => this.changeView("passwords")}>
							<Passwords class="quick-link-icon" />
							<div>
								<h3>Password manager</h3>
								<p className="quick-link-text">Save things so that you can forget them!</p>
							</div>
						</div>
						<div className="quick-link" onClick={() => this.changeView("knowledge")}>
							<Knowledge class="quick-link-icon" />
							<div>
								<h3>Knowledge</h3>
								<p className="quick-link-text">Useful if you have a silly brain.</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		);
	}
}