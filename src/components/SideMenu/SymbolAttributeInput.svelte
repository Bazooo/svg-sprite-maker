<script lang="ts">
    import type { SymbolAttribute } from '../../types/symbolAttribute'
    import { mdiDelete } from '@mdi/js'
    import { commands } from '../../types/bindings'

    export let symbolId: string
    export let symbolAttribute: SymbolAttribute

    const removeSymbolAttribute = async () => {
        await commands.removeSymbolAttribute(symbolId, symbolAttribute.key)
    }

    const updateSymbolAttribute = async () => {
        await commands.updateSymbolAttribute(symbolId, symbolAttribute.key, symbolAttribute.value)
    }
</script>

<div class="flex gap-2">
    <label class="flex grow gap-1 rounded bg-slate-200 p-1 focus-within:bg-slate-300 hover:bg-slate-300 dark:bg-slate-800 focus-within:dark:bg-slate-700 hover:dark:bg-slate-700">
        <span class="rounded bg-slate-400 px-2 py-1 text-xs text-white">{symbolAttribute.key}</span>
        <input type="text" class="w-full bg-transparent outline-0" bind:value={symbolAttribute.value} on:change={updateSymbolAttribute} />
    </label>
    <button class="text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300" on:click={removeSymbolAttribute}>
        <svg class="h-6 w-6 fill-current">
            <path d={mdiDelete} />
        </svg>
    </button>
</div>
