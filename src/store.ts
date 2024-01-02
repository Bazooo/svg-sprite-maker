import { derived, type Readable, writable } from 'svelte/store'
import type { SvgSymbol } from './types/symbol'
import type { SymbolAttribute } from './types/symbolAttribute'
import type { ApplicationConfigSettings } from './types/bindings'
import { commands } from './types/bindings'

const footerWindows = [
    // windows
    'settings',
    'shortcuts',
] as const

export type FooterWindow = (typeof footerWindows)[number]

export const sprite = writable<string | undefined>()
export const symbolIds = writable<string[]>([])
export const activeSymbolId = writable<string | undefined>()
export const applicationSettings = writable<ApplicationConfigSettings | undefined>()
export const openedFooterWindow = writable<FooterWindow | undefined>()

export const activeSymbol: Readable<SvgSymbol | undefined> = derived([activeSymbolId, sprite], ([$activeSymbolId], set) => {
    if ($activeSymbolId) {
        commands.getSvgSymbol($activeSymbolId).then((symbol) => set(symbol ?? undefined))
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
