import { Component } from 'react';

import './button.css';

export class Button extends Component {
    clicked() {
        if (this.props.disabled)
            return;

        if (this.props.clicked)
            this.props.clicked();
    }

    render() {
        const _class = this.props.class ? this.props.class : "";
        const active = this.props.active ? "active" : "";
        const disabled = this.props.disabled ? "disabled" : "";

        return (
            <div className={`main-button ${_class} ${active} ${disabled}`} onClick={() => this.clicked()}>
                { this.props.children}
            </div>
        );
    }
}

export class LinkButton extends Component {
    clicked() {
        if (this.props.disabled)
            return;

        if (this.props.clicked)
            this.props.clicked();
    }

    render() {
        const _class = this.props.class ? this.props.class : "";
        const active = this.props.active ? "active" : "";
        const disabled = this.props.disabled ? "disabled" : "";

        return (
            <div className={`link-button ${_class} ${active} ${disabled}`} onClick={() => this.clicked()}>
                { this.props.children}
            </div>
        );
    }
}