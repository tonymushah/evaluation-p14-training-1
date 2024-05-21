import type { Preview } from '@storybook/svelte';
import { withThemeByDataAttribute } from '@storybook/addon-themes';
import '../app/app.css';

const preview: Preview = {
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i
			}
		}
	}
};

export const decorators = [
	withThemeByDataAttribute({
		themes: {
			light: 'light',
			dark: 'dark'
		},
		defaultTheme: 'light',
		attributeName: 'data-mode'
	})
];

export default preview;
