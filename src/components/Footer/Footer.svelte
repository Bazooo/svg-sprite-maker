<script>
    import { mdiCog } from '@mdi/js'
    import { settingsWindowOpen } from '../../store.ts'

    const toggleSettingsWindow = () => {
        settingsWindowOpen.set(!$settingsWindowOpen)
    }
</script>

<footer class="flex items-center border-t border-slate-300 bg-slate-200 px-2 dark:border-slate-700 dark:bg-slate-800">
    <button class="h-full px-2 hover:bg-slate-300 active:bg-slate-400 hover:dark:bg-slate-700 active:dark:bg-slate-600" class:bg-slate-300={$settingsWindowOpen} class:dark:bg-slate-700={$settingsWindowOpen} on:click={toggleSettingsWindow}>
        <svg class="h-5 w-5" viewBox="0 0 24 24">
            <path d={mdiCog} />
        </svg>
    </button>
    <span class="grow" />
    <kbd>Ctrl + S</kbd>
</footer>
