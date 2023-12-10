import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import sequence from 'svelte-sequential-preprocessor'

export default {
    preprocess: sequence([
        vitePreprocess(),
    ]),
}
