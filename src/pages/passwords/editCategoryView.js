import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { LoadingIndicator } from '../../components/loading-indicator';
import { Button } from '../../components/button';
import { CheckBox } from '../../components/checkbox';
import { InputBox } from '../../components/input-box';
import { Error } from '../../components/error';

import { PasswordService } from '../../services/passwords';

export class EditCategoryView extends Component {
	constructor(props) {
		super(props);
		this.passwordService = new PasswordService();

		this.state = {
			loading: false,
			error: "",
			title: "",
			titleError: "",
			confirmReset: false,
		};
	}

	async componentDidMount() {
		this.setState({
			loading: true,
		});

		const [loaded, category] = await this.passwordService.loadCategory(this.props.categoryId);
		if (!loaded) {
			return this.setState({
				loading: false,
				error: "Failed to load category",
			});
		}

		this.setState({
			loading: false,
			title: category.title,
		});
	}

	changeView(view, id) {
		this.props.changeView(view, id);
	}

	changeTitle(value) {
		let error = "";
		if (!value) {
			error = "No name entered"
		}

		this.setState({
			title: value,
			titleError: error,
		});
	}

	async saveCategory() {
		this.setState({
			loading: true,
		});

		const saved = await this.passwordService.saveCategory(this.props.categoryId, this.state.title);
		if (!saved) {
			return this.setState({
				loading: false,
				error: "Failed to save category",
			});
		}

		this.changeView("list");
	}

	async removeCategory() {
		this.setState({
			loading: true,
		});

		const removed = await this.passwordService.removeCategory(this.props.categoryId);
		if (!removed) {
			return this.setState({
				loading: false,
				error: "Failed to remove category",
			});
		}

		this.changeView("list");
	}

	render() {
		if (this.state.error) {
			return (
				<>
					<PageHeader
						title="Edit category" 
						description="Categories are used to group similar passwords together so that it's easier to overview.">
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

		const disabled = !this.state.title || !!this.state.titleError;

		return (
			<>
				<PageHeader 
					title="Edit category" 
                    description="Categories are used to group similar passwords together so that it's easier to overview.">
				</PageHeader>

				<LoadingIndicator loading={this.state.loading}>
					<InputBox
						label="Name"
						placeholder="Enter name"
						mandatory={true}
						value={this.state.title}
						valueChanged={value => this.changeTitle(value)}
						error={this.state.titleError}>
					</InputBox>
					
					<div className="button-grid">
						<Button disabled={disabled} clicked={() => this.saveCategory()}>
							Save
						</Button>
						<Button clicked={() => this.changeView("list")}>
							Back
						</Button>
					</div>

					<Error 
						title="Danger" 
						text="Removing this category will also remove all of its passwords, it's not possible to restore any passwords once they have been removed.">
						<CheckBox 
							label="Confirm that I want to remove this category"
							checked={this.state.confirmRemove} 
							toggled={value => this.setState({confirmRemove: value})}>
						</CheckBox>

						<Button class="error-button" clicked={() => this.removeCategory()} disabled={!this.state.confirmRemove}>
							Remove
						</Button>
					</Error>
				</LoadingIndicator>
			</>
		);
	}
}