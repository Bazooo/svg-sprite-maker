<script lang="ts">
    import { mdiPlus } from '@mdi/js'
    import { invoke } from '@tauri-apps/api'
    import { activeSymbolId } from '../../store'

    let key = ''
    let value = ''

    const addAttribute = async () => {
        await invoke('update_symbol_attribute', {
            symbolId: $activeSymbolId,
            key,
            value,
        })

        key = ''
        value = ''
    }
</script>

<div class="flex flex-col gap-2 rounded bg-slate-200 p-2 dark:bg-slate-800">
    <span class="text-xs font-bold uppercase">Add new attribute</span>
    <label>
        <span>Key</span>
        <input type="text" bind:value={key} />
    </label>
    <label>
        <span>Value</span>
        <input type="text" bind:value />
    </label>
    <button class="flex items-center justify-center gap-1 rounded bg-slate-400 p-1 text-sm text-white hover:bg-slate-500 disabled:bg-slate-300 disabled:dark:bg-slate-700" on:click={addAttribute} disabled={!key || !value}>
        <svg class="h-4 w-4" viewBox="0 0 24 24">
            <path d={mdiPlus} />
        </svg>
        <span>Add</span>
    </button>
</div>

<style lang="postcss">
    label {
        @apply flex items-center gap-1 rounded border border-slate-400 p-1 focus-within:border-slate-500 hover:border-slate-500;

        & > span {
            @apply whitespace-nowrap rounded bg-slate-400 px-2 py-1 text-xs text-white;
        }

        & > input {
            @apply w-full grow rounded bg-transparent outline-0;
        }
    }
</style>
