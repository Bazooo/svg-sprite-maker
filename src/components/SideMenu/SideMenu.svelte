<script lang="ts">
    import type { SvgSymbol } from '../../types/symbol'
    import { invoke } from '@tauri-apps/api'
    import { activeSymbolId } from '../../store'

    let activeSymbol: SvgSymbol | undefined

    const setActiveSymbolId = async (symbolId: string | undefined) => {
        activeSymbol = await invoke<SvgSymbol>('get_svg_symbol', { symbolId })
    }

    const deleteSymbol = (symbolId: string) => async () => {
        await invoke('delete_svg_symbol', { symbolId })
        activeSymbolId.set(undefined)
        activeSymbol = undefined
    }

    $: if ($activeSymbolId) {
        setActiveSymbolId($activeSymbolId)
    }
</script>

<aside class="flex w-1/3 min-w-[240px] max-w-xs shrink-0 flex-col bg-slate-100 p-4">
    {#if $activeSymbolId && activeSymbol}
        <div class="p-3">
            <svg class="w-full fill-current">
                <use href="#{$activeSymbolId}" />
            </svg>
        </div>
        <div class="flex flex-col">
            <span>id: {activeSymbol.id}</span>
            <span>attributes: {JSON.stringify(activeSymbol.attributes)}</span>
        </div>
        <button on:click={deleteSymbol($activeSymbolId)}>Delete</button>
    {:else}
        <div class="flex h-full w-full items-center justify-center">
            <span>Select a symbol</span>
        </div>
    {/if}
</aside>
