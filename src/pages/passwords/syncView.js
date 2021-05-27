import { v4 as uuidv4 } from 'uuid';
import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { Button } from '../../components/button';
import { CheckBox } from '../../components/checkbox';
import { Error } from '../../components/error';

import { WebRtcService } from '../../services/webrtc';
import { decrypt, loadKey } from '../../services/crypto';
import { PasswordService } from '../../services/passwords';

import './sync.css';

export class SyncView extends Component {
	constructor(props) {
		super(props);
		this.passwordService = new PasswordService();

		this.state = {
			state: "not-connected",
			confirmReset: false,
		};
	}

	async componentDidMount() {
		const [keyLoaded, key] = loadKey();
		if (!keyLoaded)
			return;

		const id = uuidv4({random: key});

		this.webRtcService = new WebRtcService(id);
		this.webRtcService.connected = () => {
			this.setState({
				state: "connected",
			});

			this.sendPasswords();
		};
		this.webRtcService.disconnected = () => {
			this.setState({
				state: "not-connected",
			});
		};
		this.webRtcService.message = msg => {
			const {type, data} = JSON.parse(msg);

			if (type === "sync_passwords") {
				return this.syncPasswords(data);
			}
		};
	}

	componentWillUnmount() {
		this.webRtcService.closeConnection();
	}

	changeView(view, id) {
		this.props.changeView(view, id);
	}

	resetData() {
		sessionStorage.removeItem("key");
		localStorage.removeItem("verification");
		localStorage.removeItem("passwords");
		this.props.logout();
	}

	sendPasswords() {
		this.setState({
			state: "syncing",
		});

		const encryptedDataEntry = localStorage.getItem("passwords");
		if (!encryptedDataEntry) {
			return this.setState({
				state: "finished",
			});
		}
	
		this.webRtcService.sendMessage(JSON.stringify({
			type: "sync_passwords",
			data: encryptedDataEntry,
		}));
	}

	async syncPasswords(data) {
		const [keyLoaded, key] = loadKey();
		if (!keyLoaded)
			return;

		const {iv, bytes} = JSON.parse(data);

		const [decrypted, decryptedData] = await decrypt(key, new Uint8Array(iv), new Uint8Array(bytes));
		if (!decrypted)
			return;

		const passwords = JSON.parse(decryptedData);

		this.passwordService.mergePasswords(passwords);

		this.setState({
			state: "finished",
		});
	}

	render() {
		return (
            <div className="sync-data-page">
				<PageHeader 
					title="Sync data"
					description="To sync data between device both of them needs to have the same master password and then enter this page.">
				</PageHeader>

				{ this.renderStatus() }

				<div className="button-grid">
					<div></div>
					<Button class="error-button" clicked={() => this.changeView("list")}>
						Back
					</Button>
				</div>

                <Error title="Reset data" text="It is not possible to restore any data once the device has been reset.">
					<CheckBox 
						label="Confirm that I want to reset this device data"
						checked={this.state.confirmReset} 
						toggled={value => this.setState({confirmReset: value})}>
					</CheckBox>

                    <Button class="error-button" clicked={() => this.resetData()} disabled={!this.state.confirmReset}>
                        Reset device data
                    </Button>
                </Error>
			</div>
		);
	}

	renderStatus() {
		if (this.state.state === "syncing") {
			return (
				<div className="connection-status">
					<div className="connection-status-icon syncing"></div>
					<span>Syncing</span>
				</div>
			);
		}

		if (this.state.state === "finished") {
			return (
				<div className="connection-status">
					<div className="connection-status-icon finished"></div>
					<span>Finished syncing passwords</span>
				</div>
			);
		}

		if (this.state.state === "connected") {
			return (
				<div className="connection-status">
					<div className="connection-status-icon connected"></div>
					<span>Connected</span>
				</div>
			);
		}
		
		return (
			<div className="connection-status">
				<div className="connection-status-icon not-connected"></div>
				<span>Not connected</span>
			</div>
		);
	}
}