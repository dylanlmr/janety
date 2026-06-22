import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		fs: {
			allow: ['../../']
		}
	},
	ssr: {
		noExternal: ['@repo/core', '@repo/features', '@repo/state', '@repo/ui']
	},
	optimizeDeps: {
		exclude: ['@repo/core', '@repo/features', '@repo/state', '@repo/ui']
	}
});
