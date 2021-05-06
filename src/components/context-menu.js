import { Component } from 'react';

import './context-menu.css';

export class ContextMenu extends Component {
    constructor(props) {
        super(props);

        this.state = {
            open: false,
        };
    }

    setOpen(event, value) {
        event.stopPropagation();
        this.setState({
            open: value,
        });
    }

	render() {
        const opened = this.state.open ? " open" : "";

		return (
            <div className={`context-menu ${opened}`} onClick={e => this.setOpen(e, true)}>
                <div className="context-menu-backdrop" onClick={e => this.setOpen(e, false)}></div>
                { this.props.children }
            </div>
        );
	}
}

export class ContextMenuContent extends Component {
	render() {
		return (
            <div className="context-menu-content">
                { this.props.children }
            </div>
        );
	}
}