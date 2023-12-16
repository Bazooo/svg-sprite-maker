import {invoke} from "@tauri-apps/api";
import {activeSymbolId, sprite} from "./store";

const resetAppState = async () => {
    const isReset = await invoke<boolean>('reset_app_state')

    if (!isReset) {
        return
    }

    activeSymbolId.set(undefined)
    sprite.set(undefined)
}

type Shortcuts = Record<string, () => Promise<void>>

const ctrlShortcuts: Shortcuts = {
    's': () => invoke('save'),
    'n': resetAppState,
}

const ctrlShiftShortcuts: Shortcuts = {
    's': () => invoke('save_new_file')
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
