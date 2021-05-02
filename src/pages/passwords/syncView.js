import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { Button } from '../../components/button';
import { Error } from '../../components/error';

import './sync.css';

export class SyncView extends Component {
	resetData() {

	}

	render() {
		return (
            <div className="sync-data-page">
				<PageHeader 
					title="Sync data"
					description="To sync data between device both of them needs to have the same master password and then enter this page.">
				</PageHeader>

				{/* { self.render_connection_status() } */}

                <Error title="Danger" text="Resetting the data of this device will remove all passwords and knowledge.">
                    <Button class="error-button" clicked={() => this.resetData()}>
                        Reset data
                    </Button>
                </Error>
			</div>
		);
	}
}