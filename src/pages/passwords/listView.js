import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { ContextMenu, ContextMenuContent } from '../../components/context-menu';
import { LoadingIndicator } from '../../components/loading-indicator';
import { Button, LinkButton } from '../../components/button';
import { InputBox } from '../../components/input-box';
import { Error } from '../../components/error';

import { Cog } from '../../icons/cog';
import { Key } from '../../icons/key';

import { PasswordService } from '../../services/passwords';
import { writeToClipboard } from '../../services/clipboard';

export class ListView extends Component {
	constructor(props) {
		super(props);
		this.passwordService = new PasswordService();
		this.globalHotKeys = this.handleGlobalHotkey.bind(this);

		this.state = {
			loading: true,
			passwords: [],
			error: "",
			searchText: "",
		};
	}

	async componentDidMount() {
		document.addEventListener("keyup", this.globalHotKeys);

		this.setState({
			error: "",
			loading: true,
		});

		const [loaded, passwords] = await this.passwordService.listPasswords();
		if (!loaded) {
			return this.setState({
				error: "Failed to load passwords",
				loading: false,
			});
		}

		this.setState({
			passwords: passwords,
			loading: false,
		});
	}

	componentWillUnmount() {
		document.removeEventListener("keyup", this.globalHotKeys);
	}

	handleGlobalHotkey(event) {
		if (event.key === "q" && event.ctrlKey) {
			const element = document.querySelector(".search-box input");
			if (element) {
				element.focus();
			}
		}
		if (event.key === "Escape") {
			this.setState({
				searchText: "",
			});
		}
	}

	changeView(view, id) {
		this.props.changeView(view, id);
	}

	async copyDescription(passwordId) {
		const [loaded, password] = await this.passwordService.loadPassword(passwordId);
		if (!loaded)
			return;

		writeToClipboard(password.description);
	}

	async copyPassword(passwordId) {
		const [loaded, password] = await this.passwordService.loadPassword(passwordId);
		if (!loaded)
			return;

		writeToClipboard(password.password);
	}

	logout() {
		sessionStorage.removeItem("key");
		this.props.logout();
	}

	render() {
		if (this.state.error) {
			return (
				<>
					<PageHeader
						title="Password manager"
						description="Handle your passwords with ease.">
					</PageHeader>

					<Error
						title="Error"
						text={this.state.error}>
					</Error>
				</>
			);
		}

		let filteredPasswords = JSON.parse(JSON.stringify(this.state.passwords));
		if (this.state.searchText) {
			for (const category of filteredPasswords) {
				category.passwords = [...category.passwords].filter(password => password.name.toLowerCase().indexOf(this.state.searchText.toLowerCase()) > -1);
			}

			filteredPasswords = filteredPasswords.filter(category => category.passwords.length);			
		}

		return (
			<>
				<PageHeader
					title="Password manager"
					description="Handle your passwords with ease.">
					<ContextMenu>
						<Cog class="page-header-icon" />
						<ContextMenuContent>
							<Button clicked={() => this.changeView("add_category")}>Add category</Button>
							<Button clicked={() => this.changeView("sync")}>Sync data</Button>
							<Button clicked={() => this.logout()}>Logout</Button>
						</ContextMenuContent>
					</ContextMenu>
				</PageHeader>

				<InputBox
					class="search-box"
					placeholder="Search for passwords (Ctrl + Q)"
					value={this.state.searchText}
					valueChanged={value => this.setState({ searchText: value })}
					aborted={() => this.setState({ searchText: "" })}>
				</InputBox>

				<LoadingIndicator loading={this.state.loading}>
					{!this.state.searchText && filteredPasswords.length === 0 &&
						<div className="passwords-empty">
							<LinkButton clicked={() => this.changeView("add_category")}>
								Add category
							</LinkButton>
						</div>
					}
					{this.state.searchText && filteredPasswords.length === 0 &&
						<div className="passwords-empty">
							<p>No matches found</p>
						</div>
					}

					{filteredPasswords.map((category, index) => this.renderCategory(index, category))}
				</LoadingIndicator>
			</>
		);
	}

	renderCategory(index, category) {
		return (
			<div key={index}>
				<div className="category">
					<h2 className="category-title">{category.title}</h2>
					<ContextMenu>
						<Cog class="category-icon" />
						<ContextMenuContent>
							<Button clicked={() => this.changeView("add_password", category.id)}>
								Add password
							</Button>
							<Button clicked={() => this.changeView("edit_category", category.id)}>
								Edit category
							</Button>
						</ContextMenuContent>
					</ContextMenu>
				</div>

				{!this.state.searchText && category.passwords.length === 0 &&
					<div className="passwords-empty">
						<LinkButton clicked={() => this.changeView("add_password", category.id)}>
							Add password
						</LinkButton>
					</div>
				}

				<div className="category-items">
					{category.passwords.map((password, index) => this.renderPassword(index, password))}
				</div>
			</div>
		);
	}

	renderPassword(index, password) {
		return (
            <div key={index} className="password animation-fade">
                <div>
                    <h3 className="password-title">
                        <span onClick={() => this.changeView("edit_password", password.id)}>{password.name}</span>
                    </h3>
                    <p className="password-description animation-highlight">
                        <span onClick={() => this.copyDescription(password.id)}>{password.description}</span>
                    </p>
                </div>
                <div>
					<Key class="password-icon" clicked={() => this.copyPassword(password.id)} />
                </div>
			</div>
		);
	}
}