import { activeSymbolIds, sprite } from './store'
import { commands } from './types/bindings'

const resetAppState = async () => {
    const isReset = await commands.resetAppState()

    if (!isReset) {
        return
    }

    activeSymbolIds.set([])
    sprite.set(undefined)
}

type Shortcuts = Record<string, () => Promise<void> | Promise<null>>

const ctrlShortcuts: Shortcuts = {
    s: () => commands.save(),
    n: resetAppState,
}

const ctrlShiftShortcuts: Shortcuts = {
    s: () => commands.saveNewFile(),
}

export const handleShortcut = async (event: KeyboardEvent) => {
    if (event.ctrlKey) {
        if (event.shiftKey) {
            await ctrlShiftShortcuts[event.key]?.()
        } else {
            await ctrlShortcuts[event.key]?.()
        }
    }
}
