import { writable } from 'svelte/store'

export const sprite = writable<string | undefined>()
export const symbolIds = writable<string[]>([])
export const activeSymbolId = writable<string | undefined>()
