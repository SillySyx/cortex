import { Component } from 'react';

import './error.css';

export class Error extends Component {
    render() {
        return (
            <div class="error-message">
                { this.props.title &&
                    <h2 class="error-message-title">{this.props.title}</h2>
                }
                { this.props.text &&
                    <p class="error-message-text">{this.props.text}</p>
                }
                { this.props.children &&
                    <div class="error-message-content">
                        {this.props.children}
                    </div>
                }
            </div>
        );
    }
}


















