<script lang="ts">
    import { mdiCog, mdiKeyboard } from '@mdi/js'
    import { openedFooterWindow } from '../../store.ts'
    import type { FooterWindow } from '../../store.ts'

    const toggleWindow = (window: FooterWindow) => () => {
        if ($openedFooterWindow === window) {
            openedFooterWindow.set(undefined)
        } else {
            openedFooterWindow.set(window)
        }
    }

    $: settingsWindowOpened = $openedFooterWindow === 'settings'
    $: shortcutsWindowOpened = $openedFooterWindow === 'shortcuts'
</script>

<footer class="px-2bg-slate-200 flex items-center border-t border-slate-300 dark:border-slate-700 dark:bg-slate-800">
    <button class:bg-slate-300={settingsWindowOpened} class:dark:bg-slate-700={settingsWindowOpened} on:click={toggleWindow('settings')}>
        <svg class="h-5 w-5" viewBox="0 0 24 24">
            <path d={mdiCog} />
        </svg>
    </button>
    <span class="grow" />
    <button class:bg-slate-300={shortcutsWindowOpened} class:dark:bg-slate-700={shortcutsWindowOpened} on:click={toggleWindow('shortcuts')}>
        <svg class="h-5 w-5" viewBox="0 0 24 24">
            <path d={mdiKeyboard} />
        </svg>
    </button>
</footer>

<style lang="postcss">
    button {
        @apply h-full px-2 py-1 hover:bg-slate-300 active:bg-slate-400 hover:dark:bg-slate-700 active:dark:bg-slate-600;
    }
</style>
