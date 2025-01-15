import adapter from '@sveltejs/adapter-auto'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		typescript: {
			config: (c) => {
				c.include.push('../vitest-setup.ts')
			},
		},
	},
}

export default config
