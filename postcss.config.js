import tailwind from 'tailwindcss'
import tailwindConfig from './tailwind.config.js'
import autoprefixer from 'autoprefixer'
import postcssImport from 'postcss-import'
import tailwindNesting from 'tailwindcss/nesting/index.js'

export default {
    plugins: [
        postcssImport,
        tailwindNesting,
        tailwind(tailwindConfig),
        autoprefixer,
    ],
}
