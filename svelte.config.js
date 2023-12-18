import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import sequence from 'svelte-sequential-preprocessor'

/** @type {import('@sveltejs').Config} */
const config = {
    preprocess: sequence([
        vitePreprocess(),
    ]),
}

export default config
