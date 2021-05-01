import { Component, createRef } from 'react';

import './input-box.css';

export class InputBox extends Component {
    constructor(props) {
        super(props);
        this.ref = createRef();
    }

    componentDidMount() {
        if (this.props.focus) {
            this.ref.current.focus();
        }
    }

    updateValue(value) {
        if (this.props.valueChanged) {
            this.props.valueChanged(value);
        }
    }

    keyPressed(event) {
        if (event.key === "Enter" && this.props.submitted) {
            this.props.submitted();
        }
        if (event.key === "Escape" && this.props.aborted) {
            this.props.aborted();
        }
    }

    render() {
        return (
            <div className={`input-box ${this.props.class}`}>
                {this.props.label &&
                    <div className="input-box-label">
                        <label>{this.props.label}</label>
                        {this.props.showErrorIndicator && !this.props.value &&
                            <span className="input-box-error-indicator">*</span>
                        }
                    </div>
                }

                <div className="input-box-container">
                    <input
                        ref={this.ref}
                        type={this.props.password ? "password" : "text"}
                        value={this.props.value}
                        placeholder={this.props.placeholder}
                        onChange={e => this.updateValue(e.target.value)}
                        onKeyPress={e => this.keyPressed(e)}>
                    </input>

                    {this.props.children}
                </div>
                { this.props.error &&
                    <div className="input-box-error">
                        {this.props.error}
                    </div>
                }
            </div>
        );
    }
}

export class TextBox extends InputBox {
    render() {
        return (
            <div className={`input-box ${this.props.class}`}>
                {this.props.label &&
                    <div className="input-box-label">
                        <label>{this.props.label}</label>
                        {this.props.showErrorIndicator && !this.props.value &&
                            <span className="input-box-error-indicator">*</span>
                        }
                    </div>
                }

                <div className="input-box-container">
                    <textarea 
                        ref={this.ref}
                        value={this.props.value}
                        placeholder={this.props.placeholder}
                        onChange={e => this.updateValue(e.target.value)}
                        onKeyPress={e => this.keyPressed(e)}>
                    </textarea>
                </div>
                { this.props.error &&
                    <div className="input-box-error">
                        {this.props.error}
                    </div>
                }
            </div>
        );
    }
}