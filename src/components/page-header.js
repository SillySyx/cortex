import { Component } from 'react';

import './page-header.css';

export class PageHeader extends Component {
    render() {
        return (
            <div className="page-header">
                { this.props.children &&
                    <div className="page-header-content">
                        {this.props.children}
                    </div>
                }

                <h1 className="page-header-title">{this.props.title}</h1>

                { this.props.description &&
                    <p className="page-header-description">{this.props.description}</p>
                }
            </div>
        );
    }
}