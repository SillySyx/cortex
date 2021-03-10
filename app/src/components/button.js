import React from 'react';
import './button.css'

export class Button extends React.Component {
    render() {
        return (
            <div className={"main-button" + (this.props.active ? " active" : "")}
                onClick={this.props.clicked}>

                {this.props.children}
            </div>
        );
    }
}