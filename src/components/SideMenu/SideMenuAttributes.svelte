<script lang="ts">
    import { activeSymbol, activeSymbolAttributes, activeSymbolId, activeSymbolIds } from '../../store'
    import SymbolAttributeInput from './SymbolAttributeInput.svelte'
    import AddAttributeForm from './AddAttributeForm.svelte'
    import { commands } from '../../types/bindings'
    import type { ChangeEventHandler } from 'svelte/elements'

    const updateSymbolId: ChangeEventHandler<HTMLInputElement> = async (event) => {
        if (typeof $activeSymbolId === 'undefined') {
            return
        }

        await commands.updateSymbolAttribute($activeSymbolId, 'id', event.currentTarget.value)

        activeSymbolIds.set([event.currentTarget.value])
    }
</script>

<div class="flex grow flex-col gap-2">
    <input type="text" class="w-full rounded bg-slate-200 px-2 py-1 outline-0 focus-within:bg-slate-300 hover:bg-slate-300 dark:bg-slate-800 focus-within:dark:bg-slate-700 hover:dark:bg-slate-700" value={$activeSymbol?.id} on:change={updateSymbolId} />
    <span class="text-lg font-bold">Attributes</span>
    <div class="flex grow flex-col gap-2 overflow-y-auto">
        {#if $activeSymbolAttributes.length > 0}
            {#each $activeSymbolAttributes as symbolAttribute}
                <SymbolAttributeInput symbolId={$activeSymbolId ?? ''} {symbolAttribute} />
            {/each}
        {:else}
            <span class="text-sm italic text-neutral-400">No attributes</span>
        {/if}

        <AddAttributeForm />
    </div>
</div>
