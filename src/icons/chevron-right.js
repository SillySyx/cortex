import { Component } from 'react';

export class ChevronRight extends Component {
	render() {
		return (
            <svg className={this.props.class} viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg" fill="currentColor">
				<polygon points="79.093,0 48.907,30.187 146.72,128 48.907,225.813 79.093,256 207.093,128" />
			</svg>
		);
	}
}