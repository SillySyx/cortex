import { Component } from 'react';

import './checkbox.css';

export class CheckBox extends Component {
    constructor(props) {
        super(props);

        this.state = {
            checked: props.value,
        };
    }

    toggle() {
        const checked = !this.state.checked;
        this.setState({checked: checked});

        if (this.props.toggled) {
            this.props.toggled(checked);
        }
    }

    render() {
        const checked = this.state.checked ? "checked" : "";

        return (
            <div className={`check-box ${checked}`}>
                <div className="switch-container" onClick={() => this.toggle()}>
                    <div className="switch"></div>
                </div>

                { this.props.label &&
                    <label>{ this.props.label }</label>
                }
            </div>
        );
    }
}