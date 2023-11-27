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
        <div class="p-6 relative">
            <div class="relative aspect-square w-full z-10">
                <svg class="absolute h-full w-full z-10">
                    <defs>
                        <pattern id="smallGrid" width="2%" height="2%" patternUnits="userSpaceOnUse">
                            <path d="M 8 0 L 0 0 0 8" fill="none" stroke="gray" stroke-width="0.5" />
                        </pattern>
                        <pattern id="grid" width="20%" height="20%" patternUnits="userSpaceOnUse">
                            <rect width="80" height="80" fill="url(#smallGrid)" />
                            <path d="M 80 0 L 0 0 0 80" fill="none" stroke="gray" stroke-width="0.5" />
                        </pattern>
                    </defs>

                    <rect width="100%" height="100%" fill="url(#grid)" />
                    <line x1="100%" y1="0" x2="100%" y2="100%" stroke="gray" />
                    <line x1="0" y1="100%" x2="100%" y2="100%" stroke="gray" />
                </svg>
                <svg class="h-full w-full fill-current">
                    <use href="#{$activeSymbolId}" />
                </svg>
            </div>
            <svg class="absolute h-full w-full top-0 left-0 z-0">
                <defs>
                    <pattern id="transparent-grid" width="20" height="20" patternUnits="userSpaceOnUse">
                        <rect fill="black" x="0" y="0" width="10" height="10" opacity="0.1"/>
                        <rect fill="white" x="10" y="0" width="10" height="10"/>
                        <rect fill="black" x="10" y="10" width="10" height="10" opacity="0.1"/>
                        <rect fill="white" x="0" y="10" width="10" height="10"/>
                    </pattern>
                </defs>
                <rect fill="url(#transparent-grid)" x="0" y="0" width="100%" height="100%"/>
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
