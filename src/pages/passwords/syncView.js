import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { Button } from '../../components/button';
import { Error } from '../../components/error';

import './sync.css';

export class SyncView extends Component {
	changeView(view, id) {
		this.props.changeView(view, id);
	}

	resetData() {
		sessionStorage.removeItem("key");
		localStorage.removeItem("verification");
		localStorage.removeItem("passwords");
		this.props.logout();
	}

	render() {
		return (
            <div className="sync-data-page">
				<PageHeader 
					title="Sync data"
					description="To sync data between device both of them needs to have the same master password and then enter this page.">
				</PageHeader>

				{/* { self.render_connection_status() } */}

				<div className="button-grid">
					<div></div>
					<Button class="error-button" clicked={() => this.changeView("list")}>
						Back
					</Button>
				</div>

                <Error title="Reset data" text="It is not possible to restore any data once the device has been reset.">
                    <Button class="error-button" clicked={() => this.resetData()}>
                        Reset device data
                    </Button>
                </Error>
			</div>
		);
	}
}