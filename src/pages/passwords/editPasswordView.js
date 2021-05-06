import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { LoadingIndicator } from '../../components/loading-indicator';
import { Button } from '../../components/button';
import { InputBox } from '../../components/input-box';
import { Error } from '../../components/error';

import { Copy } from '../../icons/copy';

import { PasswordService } from '../../services/passwords';
import { writeToClipboard } from '../../services/clipboard';

export class EditPasswordView extends Component {
	constructor(props) {
		super(props);
		this.passwordService = new PasswordService();

		this.state = {
			loading: true,
			error: "",
			name: "",
			nameError: "",
			description: "",
			password: "",
			passwordError: "",
		};
	}

	async componentDidMount() {
		this.setState({
			loading: true,
		});

		const [loaded, password] = await this.passwordService.loadPassword(this.props.passwordId);
		if (!loaded) {
			return this.setState({
				loading: false,
				error: "Failed to load password",
			});
		}

		this.setState({
			loading: false,
			name: password.name,
			description: password.description,
			password: password.password,
		});
	}

	changeView(view, id) {
		this.props.changeView(view, id);
	}

	changeName(value) {
		let error = "";
		if (!value) {
			error = "No name entered"
		}

		this.setState({
			name: value,
			nameError: error,
		});
	}

	changePassword(value) {
		let error = "";
		if (!value) {
			error = "No password entered"
		}

		this.setState({
			password: value,
			passwordError: error,
		});
	}

	async copyDescription() {
		writeToClipboard(this.state.description);
	}

	async copyPassword() {
		writeToClipboard(this.state.password);
	}

	async savePassword() {
		this.setState({
			loading: true,
		});

		const saved = await this.passwordService.savePassword(this.props.passwordId, this.state.name, this.state.description, this.state.password);
		if (!saved) {
			return this.setState({
				loading: false,
				error: "Failed to save password",
			});
		}

		this.changeView("list");
	}

	async removePassword() {
		this.setState({
			loading: true,
		});

		const removed = await this.passwordService.removePassword(this.props.passwordId);
		if (!removed) {
			return this.setState({
				loading: false,
				error: "Failed to remove password",
			});
		}

		this.changeView("list");
	}

	render() {
		if (this.state.error) {
			return (
				<>
					<PageHeader
						title="Edit password">
					</PageHeader>

					<Error 
						title="Error" 
						text={this.state.error}>
						<Button clicked={() => this.changeView("list")}>
							Back
						</Button>
					</Error>
				</>
			);
		}

		const disabled = !this.state.name || !!this.state.nameError || !this.state.password || !!this.state.passwordError;

		return (
			<>
				<PageHeader 
					title="Edit password">
				</PageHeader>

				<LoadingIndicator loading={this.state.loading}>
					<InputBox
						label="Name"
						placeholder="Enter name"
						mandatory={true}
						value={this.state.name}
						valueChanged={value => this.changeName(value)}
						error={this.state.nameError}>
					</InputBox>

					<InputBox
						label="Description"
						placeholder="Enter description"
						value={this.state.description}
						valueChanged={value => this.setState({description: value})}>
						<Copy class="input-box-icon" clicked={() => this.copyDescription()} />
					</InputBox>

					<InputBox
						label="Password"
						placeholder="Enter password"
						mandatory={true}
						password={true}
						value={this.state.password}
						valueChanged={value => this.changePassword(value)}
						error={this.state.passwordError}>
						<Copy class="input-box-icon" clicked={() => this.copyPassword()} />
					</InputBox>

					<div className="button-grid">
						<Button disabled={disabled} clicked={() => this.savePassword()}>
							Save
						</Button>
						<Button clicked={() => this.changeView("list")}>
							Back
						</Button>
					</div>
					
					<Error 
						title="Danger" 
						text="It's not possible to restore the password once it has been removed.">
						<Button class="error-button" clicked={() => this.removePassword()}>
							Remove
						</Button>
					</Error>
				</LoadingIndicator>
			</>
		);
	}
}