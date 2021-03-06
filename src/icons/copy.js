import { Component } from 'react';

export class Copy extends Component {
    clicked() {
        if (this.props.clicked)
            this.props.clicked();
    }

	render() {
		return (
            <svg className={this.props.class} viewBox="0 0 442 442" xmlns="http://www.w3.org/2000/svg" fill="currentColor" onClick={() => this.clicked()}>
				<polygon points="291,0 51,0 51,332 121,332 121,80 291,80" />
				<polygon points="306,125 306,195 376,195" />
				<polygon points="276,225 276,110 151,110 151,442 391,442 391,225" />
			</svg>
		);
	}
}