import { Component } from 'react';

import './loading-indicator.css';

export class LoadingIndicator extends Component {
	render() {
        if (this.props.loading) {
            return (
                <svg className="loading-indicator" viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg" fill="currentColor">
                    <path d="M64 128A64 64 0 0 1 18.34 19.16L21.16 22a60 60 0 1 0 52.8-17.17l.62-3.95A64 64 0 0 1 64 128z">
                        <animateTransform 
                            attributeName="transform"
                            attributeType="XML"
                            type="rotate"
                            from="0 64 64"
                            to="360 64 64"
                            dur="1s"
                            repeatCount="indefinite" />
                    </path>
                </svg>
            );
        }

		return this.props.children;
	}
}