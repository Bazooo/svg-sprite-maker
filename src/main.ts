import 'sanitize.css'
import './global.css'
import App from './App.svelte'
import { applicationSettings } from './store'

const app = new App({
    target: document.getElementById('app'),
})

applicationSettings.subscribe((settings) => {
    document.documentElement.classList.toggle('dark', settings?.darkMode)
})
export default app
