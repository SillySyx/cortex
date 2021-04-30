import { Component } from 'react';

import { HomePage } from './pages/home/page';
import { KnowledgePage } from './pages/knowledge/page';
import { LoginPage } from './pages/login/page';
import { PasswordsPage } from './pages/passwords/page';
import { SyncPage } from './pages/sync/page';

import { generateKey, storeKey, loadKey, encrypt, decrypt } from './services/crypto';

export class App extends Component {
	constructor(props) {
		super(props);

		this.state = {
			view: "home",
		};
	}

	async componentDidMount() {
		const [generated, key] = await generateKey();
		console.log("generateKey", generated, key);

		const stored = await storeKey(key);
		console.log("storeKey", stored);

		const [loaded, storedKey] = await loadKey();
		console.log("loadKey", loaded, storedKey);

		const [encrypted, encryptedData] = await encrypt(storedKey, "hello world");
		console.log("encrypt", encrypted, encryptedData);

		const [decrypted, decryptedData] = await decrypt(storedKey, encryptedData);
		console.log("decrypt", decrypted, decryptedData);
	}

	render() {
		// const [loggedIn, masterKey] = getMasterKey();
		// if (!loggedIn) {
		// 	return (
		// 		<LoginPage changeView={view => this.setState({ view: view })} />
		// 	);
		// }

		return (
			<div>
			{ this.state.view === "passwords" &&
				<PasswordsPage changeView={view => this.setState({ view: view })} />
			}
			{ this.state.view === "knowledge" &&
				<KnowledgePage changeView={view => this.setState({ view: view })} />
			}
			{ this.state.view === "sync" &&
				<SyncPage changeView={view => this.setState({ view: view })} />
			}
			{ this.state.view === "home" &&
				<HomePage changeView={view => this.setState({ view: view })} />
			}
			</div>
		);
	}
}