<script lang="ts">
    import { mdiClose } from '@mdi/js'
    import { openedFooterWindow } from '../../store.js'

    export let title

    const closeWindow = () => {
        openedFooterWindow.set(undefined)
    }
</script>

<aside class="flex h-1/2 shrink-0 flex-col overflow-y-auto border-t border-slate-300 bg-slate-200 dark:border-slate-700 dark:bg-slate-800">
    <div class="flex shrink-0 items-center border-b border-slate-300 bg-slate-100 px-2 py-1 dark:border-slate-700 dark:bg-slate-900">
        <span>{title}</span>
        <span class="grow" />
        <button class="h-full rounded p-1 hover:bg-slate-300 active:bg-slate-200 hover:dark:bg-slate-700 active:dark:bg-slate-800" on:click={closeWindow}>
            <svg class="h-4 w-4" viewBox="0 0 24 24">
                <path d={mdiClose} />
            </svg>
        </button>
    </div>

    <slot />
</aside>
