import { config } from '@repo/eslint-config/index.js';

export default [
	...config,
	{
		ignores: ['src-tauri/target/', 'src-tauri/gen/', '.svelte-kit/', 'build/', 'dist/']
	}
];
