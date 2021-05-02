import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { LoadingIndicator } from '../../components/loading-indicator';
import { Button } from '../../components/button';
import { InputBox } from '../../components/input-box';
import { Error } from '../../components/error';

import { Generate } from '../../icons/generate';

import { PasswordService } from '../../services/passwords';

export class AddPasswordView extends Component {
	constructor(props) {
		super(props);
		this.passwordService = new PasswordService();

		this.state = {
			loading: false,
			error: "",
			name: "",
			nameError: "",
			description: "",
			password: "",
			passwordError: "",
		};
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

	generatePassphrase() {
		this.setState({
			password: "huh?",
		});
	}

	async addPassword() {
		this.setState({
			loading: true,
			error: "",
		});

		const created = await this.passwordService.createPassword(this.props.categoryId, this.state.name, this.state.description, this.state.password);
		if (!created) {
			return this.setState({
				loading: false,
				error: "Failed to create password",
			});
		}

		this.changeView("list");
	}

	render() {
		if (this.state.error) {
			return (
				<>
					<PageHeader
						title="Add password">
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
					title="Add password">
				</PageHeader>

				<LoadingIndicator loading={this.state.loading}>
					<InputBox
						label="Name"
						placeholder="Enter name"
						focus={true}
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
					</InputBox>

					<InputBox
						label="Password"
						placeholder="Enter password"
						mandatory={true}
						value={this.state.password}
						valueChanged={value => this.changePassword(value)}
						error={this.state.passwordError}>
						<Generate class="input-box-icon" clicked={() => this.generatePassphrase()} />
					</InputBox>

					<div className="button-grid">
						<Button disabled={disabled} clicked={() => this.addPassword()}>
							Add
						</Button>
						<Button clicked={() => this.changeView("list")}>
							Back
						</Button>
					</div>
				</LoadingIndicator>
			</>
		);
	}
}