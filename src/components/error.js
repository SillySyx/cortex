import { Component } from 'react';

import './error.css';

export class Error extends Component {
    render() {
        return (
            <div className="error-message">
                { this.props.title &&
                    <h2 className="error-message-title">{this.props.title}</h2>
                }
                { this.props.text &&
                    <p className="error-message-text">{this.props.text}</p>
                }
                { this.props.children &&
                    <div className="error-message-content">
                        {this.props.children}
                    </div>
                }
            </div>
        );
    }
}


















