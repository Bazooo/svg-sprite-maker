<script lang="ts">
    import type { SvgSymbol } from '../../types/symbol'
    import { invoke } from '@tauri-apps/api'

    export let activeSymbolId: string | undefined

    let activeSymbol: SvgSymbol | undefined

    const setActiveSymbolId = async (symbolId: string) => {
        activeSymbol = await invoke<SvgSymbol>('get_svg_symbol', { symbolId })
    }

    $: if (activeSymbolId) {
        setActiveSymbolId(activeSymbolId)
    }
</script>

<aside class="flex flex-col shrink-0 w-1/3 max-w-xs min-w-[240px] bg-slate-100 p-4">
    {#if activeSymbolId && activeSymbol}
        <div class="p-3">
            <svg class="fill-current w-full">
                <use href="#{activeSymbolId}" />
            </svg>
        </div>
        <div class="flex flex-col">
            <span>id: {activeSymbol.id}</span>
            <span>attributes: {JSON.stringify(activeSymbol.attributes)}</span>
        </div>
    {:else}
        <div class="flex justify-center items-center w-full h-full">
            <span>Select a symbol</span>
        </div>
    {/if}
</aside>
