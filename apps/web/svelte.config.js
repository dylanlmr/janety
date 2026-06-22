import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	compilerOptions: {
		runes: true
	},
	kit: {
		adapter: adapter(),
		alias: {
			'#core': '../../packages/core/src',
			'#features': '../../packages/features/src',
			'#state': '../../packages/state/src',
			'#ui': '../../packages/ui/src'
		}
	}
};

export default config;
