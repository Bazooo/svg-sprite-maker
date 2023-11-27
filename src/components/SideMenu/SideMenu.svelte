<script lang="ts">
    import { invoke } from '@tauri-apps/api'
    import { activeSymbol, activeSymbolAttributes, activeSymbolId } from '../../store'
    import SymbolAttributeInput from './SymbolAttributeInput.svelte'

    const deleteSymbol = (symbolId: string) => async () => {
        await invoke('delete_svg_symbol', { symbolId })
        activeSymbolId.set(undefined)
    }
</script>

<aside class="flex w-2/5 min-w-[240px] max-w-[360px] shrink-0 flex-col gap-3 bg-slate-100 p-4">
    {#if $activeSymbol}
        <div class="p-6">
            <svg class="w-full fill-current aspect-square">
                <use href="#{$activeSymbolId}" />
            </svg>
        </div>
        <div class="flex flex-col gap-1">
            <span>id: {$activeSymbol.id}</span>
            <span class="text-lg font-bold">Attributes</span>
            {#if $activeSymbolAttributes.length > 0}
                {#each $activeSymbolAttributes as symbolAttribute}
                    <SymbolAttributeInput symbolId={$activeSymbolId} {symbolAttribute} />
                {/each}
            {:else}
                <span class="text-sm italic text-neutral-400">No attributes</span>
            {/if}
        </div>
        <button on:click={deleteSymbol($activeSymbolId)}>Delete</button>
    {:else}
        <div class="flex h-full w-full items-center justify-center">
            <span>Select a symbol</span>
        </div>
    {/if}
</aside>
