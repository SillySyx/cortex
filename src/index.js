import { StrictMode } from 'react';
import { render } from 'react-dom';

import { MainPage } from './pages/main/mainPage';

render(
	<StrictMode>
		<MainPage />
	</StrictMode>,
	document.getElementById('root')
);