import { Component } from 'react';

import { InputBox } from '../../components/input-box';
import { LoadingIndicator } from '../../components/loading-indicator';

import { Logo } from '../../icons/logo';

import { generateKeyFromSeed, storeKey, verifyKey } from '../../services/crypto';

import './login.css';

export class LoginPage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            password: "",
            error: "",
            loading: false,
        };
    }

    async login() {
        if (!this.state.password) {
            return;
        }

        this.setState({
            loading: true,
        });

        const [loaded, key] = await generateKeyFromSeed(this.state.password);
        if (!loaded) {
            return this.setState({
                error: "Failed to generate key from password",
                loading: false,
            });
        }

        const verified = await verifyKey(key);
        if (!verified) {
            return this.setState({
                error: "Incorrect key specified",
                loading: false,
            });
        }

        const stored = storeKey(key);
        if (!stored) {
            return this.setState({
                error: "Failed to store key",
                loading: false,
            });

        }

        this.setState({
            loading: false,
        });

        this.props.login();
    }

    render() {
        return (
            <div className="login-layout login-background">
                <div className="login-content">
                    <Logo class="login-logo" />

                    <LoadingIndicator loading={this.state.loading}>
                        <InputBox
                            class="login-input"
                            placeholder="Enter your password"
                            focus={true}
                            password={true}
                            value={this.state.value}
                            valueChanged={value => this.setState({ password: value })}
                            error={this.state.error}
                            submitted={() => this.login()}>
                        </InputBox>
                    </LoadingIndicator>

                    <p>{process.env.REACT_APP_VERSION}</p>
                </div>
            </div>
        );
    }
}