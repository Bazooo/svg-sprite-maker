import { derived, type Readable, writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api'
import type { SvgSymbol } from './types/symbol'
import type { SymbolAttribute } from './types/symbolAttribute'

export const sprite = writable<string | undefined>()
export const symbolIds = writable<string[]>([])
export const activeSymbolId = writable<string | undefined>()

export const activeSymbol: Readable<SvgSymbol | undefined> = derived([activeSymbolId, sprite], ([$activeSymbolId], set) => {
    if ($activeSymbolId) {
        invoke<SvgSymbol>('get_svg_symbol', { symbolId: $activeSymbolId }).then((symbol) => set(symbol))
    } else {
        set(undefined)
    }
})

export const activeSymbolAttributes = derived(
    activeSymbol,
    ($activeSymbol, set) => {
        const symbolAttributes = Object.entries($activeSymbol?.attributes ?? {})
            .filter(([key]) => key !== 'id')
            .map(([key, value]) => ({ key, value }))

        set(symbolAttributes)
    },
    [] as SymbolAttribute[],
)
