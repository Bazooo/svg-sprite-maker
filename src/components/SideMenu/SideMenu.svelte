<script lang="ts">
    import { invoke } from '@tauri-apps/api'
    import { activeSymbol, activeSymbolId } from '../../store'
    import { mdiDelete, mdiGrid, mdiSquareOpacity, mdiXml } from '@mdi/js'
    import SymbolPreviewGrid from './SymbolPreviewGrid.svelte'
    import TransparentGrid from '../TransparentGrid.svelte'
    import SideMenuAttributes from './SideMenuAttributes.svelte'
    import { commands } from '../../types/bindings'

    let showGrid = true
    let showTransparentGrid = true

    const editSymbol = (symbolId: string) => async () => {
        await commands.editSvgSymbol(symbolId)
    }

    const deleteSymbol = (symbolId: string) => async () => {
        await commands.deleteSvgSymbol(symbolId)
        activeSymbolId.set(undefined)
    }
</script>

<aside class="flex w-2/5 min-w-[240px] max-w-[360px] shrink-0 flex-col gap-3 overflow-y-auto border-l border-slate-300 bg-slate-100 p-4 dark:border-slate-700 dark:bg-slate-900">
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
                    <TransparentGrid />
                {/if}
            </div>
            <div class="flex flex-col gap-3">
                <label class="toggle-button" class:border-transparent={!showGrid} class:border-slate-300={showGrid} title="Show grid">
                    <svg class="h-6 w-6">
                        <path d={mdiGrid} />
                    </svg>
                    <input class="hidden" type="checkbox" bind:checked={showGrid} />
                </label>
                <label class="toggle-button" class:border-transparent={!showTransparentGrid} class:border-slate-300={showTransparentGrid} title="Show transparent grid">
                    <svg class="h-6 w-6">
                        <path d={mdiSquareOpacity} />
                    </svg>
                    <input class="hidden" type="checkbox" bind:checked={showTransparentGrid} />
                </label>
                <span class="grow" />
                <button class="cursor-pointer rounded border border-transparent p-1 text-blue-500 hover:border-blue-500 active:bg-blue-100 active:dark:bg-blue-900" on:click={editSymbol($activeSymbolId)} title="Edit symbol">
                    <svg class="h-6 w-6">
                        <path d={mdiXml} />
                    </svg>
                </button>
                <button class="cursor-pointer rounded border border-transparent p-1 text-red-500 hover:border-red-500 active:bg-red-100 active:dark:bg-red-900" on:click={deleteSymbol($activeSymbolId)} title="Remove symbol">
                    <svg class="h-6 w-6 fill-current">
                        <path d={mdiDelete} />
                    </svg>
                </button>
            </div>
        </div>
        <SideMenuAttributes />
    {:else}
        <div class="flex h-full w-full items-center justify-center">
            <span>Select a symbol</span>
        </div>
    {/if}
</aside>

<style lang="postcss">
    .toggle-button {
        @apply cursor-pointer rounded border p-1 hover:bg-slate-200 hover:dark:bg-slate-800;
    }
</style>
