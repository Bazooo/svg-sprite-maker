<script lang="ts">
    import { invoke } from '@tauri-apps/api'
    import { activeSymbol, activeSymbolAttributes, activeSymbolId } from '../../store'
    import SymbolAttributeInput from './SymbolAttributeInput.svelte'
    import { mdiDelete, mdiGrid, mdiSquareOpacity, mdiXml } from '@mdi/js'
    import SymbolPreviewGrid from './SymbolPreviewGrid.svelte'
    import SymbolPreviewTransparentGrid from './SymbolPreviewTransparentGrid.svelte'

    let showGrid = true
    let showTransparentGrid = true

    const updateSymbolId = async (event: InputEvent) => {
        const target = event.currentTarget as HTMLInputElement

        await invoke('update_symbol_attribute', {
            symbolId: $activeSymbolId,
            key: 'id',
            value: target.value,
        })

        activeSymbolId.set(target.value)
    }

    const editSymbol = (symbolId: string) => async () => {
        await invoke('edit_svg_symbol', { symbolId })
    }

    const deleteSymbol = (symbolId: string) => async () => {
        await invoke('delete_svg_symbol', { symbolId })
        activeSymbolId.set(undefined)
    }
</script>

<aside class="flex w-2/5 min-w-[240px] max-w-[360px] shrink-0 flex-col gap-3 bg-slate-100 p-4">
    {#if $activeSymbol}
        <div class="flex gap-2">
            <div class="relative aspect-square w-full">
                <svg class="absolute z-10 h-full w-full fill-current">
                    <use href="#{$activeSymbolId}" />
                </svg>
                {#if showGrid}
                    <SymbolPreviewGrid />
                {/if}
                {#if showTransparentGrid}
                    <SymbolPreviewTransparentGrid />
                {/if}
            </div>
            <div class="flex flex-col gap-2">
                <label class="cursor-pointer rounded border p-1 hover:bg-slate-200" class:border-transparent={!showGrid} class:border-slate-300={showGrid} title="Show grid">
                    <svg class="h-6 w-6">
                        <path d={mdiGrid} />
                    </svg>
                    <input class="hidden" type="checkbox" bind:checked={showGrid} />
                </label>
                <label class="cursor-pointer rounded border p-1 hover:bg-slate-200" class:border-transparent={!showTransparentGrid} class:border-slate-300={showTransparentGrid} title="Show transparent grid">
                    <svg class="h-6 w-6">
                        <path d={mdiSquareOpacity} />
                    </svg>
                    <input class="hidden" type="checkbox" bind:checked={showTransparentGrid} />
                </label>
                <span class="grow" />
                <button class="cursor-pointer rounded border border-transparent p-1 text-blue-500 hover:border-blue-500" on:click={editSymbol($activeSymbolId)} title="Edit symbol">
                    <svg class="h-6 w-6">
                        <path d={mdiXml} />
                    </svg>
                </button>
                <button class="cursor-pointer rounded border border-transparent p-1 text-red-500 hover:border-red-500" on:click={deleteSymbol($activeSymbolId)} title="Remove symbol">
                    <svg class="h-6 w-6 fill-current">
                        <path d={mdiDelete} />
                    </svg>
                </button>
            </div>
        </div>
        <div class="flex grow flex-col gap-2">
            <input type="text" class="w-full rounded bg-slate-200 px-2 py-1 outline-0 hover:bg-slate-300 focus:bg-slate-300" value={$activeSymbol.id} on:change={updateSymbolId} />
            <span class="text-lg font-bold">Attributes</span>
            <div class="flex grow flex-col gap-1 overflow-y-auto">
                {#if $activeSymbolAttributes.length > 0}
                    {#each $activeSymbolAttributes as symbolAttribute}
                        <SymbolAttributeInput symbolId={$activeSymbolId} {symbolAttribute} />
                    {/each}
                {:else}
                    <span class="text-sm italic text-neutral-400">No attributes</span>
                {/if}
            </div>
        </div>
    {:else}
        <div class="flex h-full w-full items-center justify-center">
            <span>Select a symbol</span>
        </div>
    {/if}
</aside>
