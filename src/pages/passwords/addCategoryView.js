import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { LoadingIndicator } from '../../components/loading-indicator';
import { Button } from '../../components/button';
import { InputBox } from '../../components/input-box';
import { Error } from '../../components/error';

import { PasswordService } from '../../services/passwords';

export class AddCategoryView extends Component {
	constructor(props) {
		super(props);
		this.passwordService = new PasswordService();

		this.state = {
			loading: false,
			error: "",
			title: "",
			titleError: "",
		};
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

	async addCategory() {
		this.setState({
			loading: true,
			error: "",
		});

		const created = await this.passwordService.createCategory(this.state.title);
		if (!created) {
			return this.setState({
				loading: false,
				error: "Failed to create category",
			});
		}

		this.changeView("list");
	}

	render() {
		if (this.state.error) {
			return (
				<>
					<PageHeader
						title="New category" 
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
					title="New category" 
                    description="Categories are used to group similar passwords together so that it's easier to overview.">
				</PageHeader>

				<LoadingIndicator loading={this.state.loading}>
					<InputBox
						label="Name"
						placeholder="Enter name"
						focus={true}
						mandatory={true}
						value={this.state.title}
						valueChanged={value => this.changeTitle(value)}
						error={this.state.titleError}>
					</InputBox>

					<div className="button-grid">
						<Button disabled={disabled} clicked={() => this.addCategory()}>
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