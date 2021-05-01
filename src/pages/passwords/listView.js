import { Component } from 'react';

import { PageHeader } from '../../components/page-header';
import { ContextMenu, ContextMenuContent } from '../../components/context-menu';
import { Button, LinkButton } from '../../components/button';
import { InputBox } from '../../components/input-box';
import { Error } from '../../components/error';

import { Cog } from '../../icons/cog';

export class ListView extends Component {
	constructor(props) {
		super(props);

		this.state = {
			error: "",
			searchText: "",
		};
	}

	changeView(view) {
		this.props.changeView(view);
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

		const categories = [];

		return (
            <>
				<PageHeader 
					title="Password manager" 
					description="Handle your passwords with ease.">
					<ContextMenu>
						<Cog class="page-header-icon" />
						<ContextMenuContent>
							<Button clicked={() => this.changeView("add_category")}>Add category</Button>
						</ContextMenuContent>
					</ContextMenu>
                </PageHeader>

				<InputBox 
                    class="search-box"
                    placeholder="Search for passwords"
                    value={this.state.searchText}
                    valueChanged={value => this.setState({searchText: value})}
                    aborted={() => this.setState({searchText: ""})}>
                </InputBox>

				{ !this.state.searchText && categories.length === 0 &&
					<div class="passwords-empty">
						<LinkButton clicked={() => this.changeView("add_category")}>
							Add category
						</LinkButton>
					</div>
				}

				{ categories.map((category, index) => this.renderCategory(index, category)) }
			</>
		);
	}

	renderCategory(index, category) {
		return (
			<div key={index} class="category">
                <h2 class="category-title">{category.title}</h2>
                {/* <ContextMenu>
                    <Svg class="category-icon animation-twist-grow" src="icons/cog.svg" />
                    <ContextMenuContent>
                        <Button clicked=new_password.clone()>
                            {"Add password"}
                        </Button>
                        <Button clicked=edit_category>
                            {"Edit category"}
                        </Button>
                    </ContextMenuContent>
                </ContextMenu> */}
            </div>
		);
	}
}