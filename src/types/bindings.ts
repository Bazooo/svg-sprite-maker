// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

export const commands = {
    async getSvgSymbol(symbolId: string): Promise<{ id: string; attributes: { [key in string]: string } } | null> {
        return await TAURI_INVOKE('plugin:tauri-specta|get_svg_symbol', { symbolId })
    },
    async editSvgSymbol(symbolId: string): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|edit_svg_symbol', { symbolId })
    },
    async deleteSvgSymbol(symbolId: string): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|delete_svg_symbol', { symbolId })
    },
    async saveNewFile(): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|save_new_file')
    },
    async save(): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|save')
    },
    async updateSymbolAttribute(symbolId: string, key: string, value: string): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|update_symbol_attribute', { symbolId, key, value })
    },
    async removeSymbolAttribute(symbolId: string, key: string): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|remove_symbol_attribute', { symbolId, key })
    },
    async setAutoSave(enabled: boolean): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|set_auto_save', { enabled })
    },
    async setDarkMode(enabled: boolean): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|set_dark_mode', { enabled })
    },
    async getAppSettings(): Promise<ApplicationConfigSettings> {
        return await TAURI_INVOKE('plugin:tauri-specta|get_app_settings')
    },
    async setEditorPath(path: string): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|set_editor_path', { path })
    },
    async setTransparencyGridColors(g1: [string, number] | null, g2: [string, number] | null): Promise<null> {
        return await TAURI_INVOKE('plugin:tauri-specta|set_transparency_grid_colors', { g1, g2 })
    },
    async resetAppState(): Promise<boolean> {
        return await TAURI_INVOKE('plugin:tauri-specta|reset_app_state')
    },
    async searchSymbolsById(query: string): Promise<string[]> {
        return await TAURI_INVOKE('plugin:tauri-specta|search_symbols_by_id', { query })
    },
}

/** user-defined types **/

export type ApplicationConfigSettings = { autoSaveEnabled: boolean; editorPath?: string | null; darkMode: boolean; transparencyGridColor1?: TransparencyGridColor | null; transparencyGridColor2?: TransparencyGridColor | null }
export type TransparencyGridColor = [string, number]

/** tauri-specta globals **/

import { invoke as TAURI_INVOKE } from '@tauri-apps/api'
import * as TAURI_API_EVENT from '@tauri-apps/api/event'
import { type WebviewWindowHandle as __WebviewWindowHandle__ } from '@tauri-apps/api/window'

type __EventObj__<T> = {
    listen: (cb: TAURI_API_EVENT.EventCallback<T>) => ReturnType<typeof TAURI_API_EVENT.listen<T>>
    once: (cb: TAURI_API_EVENT.EventCallback<T>) => ReturnType<typeof TAURI_API_EVENT.once<T>>
    emit: T extends null ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit> : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>
}

type __Result__<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E }

function __makeEvents__<T extends Record<string, any>>(mappings: Record<keyof T, string>) {
    return new Proxy(
        {} as unknown as {
            [K in keyof T]: __EventObj__<T[K]> & {
                (handle: __WebviewWindowHandle__): __EventObj__<T[K]>
            }
        },
        {
            get: (_, event) => {
                const name = mappings[event as keyof T]

                return new Proxy((() => {}) as any, {
                    apply: (_, __, [window]: [__WebviewWindowHandle__]) => ({
                        listen: (arg: any) => window.listen(name, arg),
                        once: (arg: any) => window.once(name, arg),
                        emit: (arg: any) => window.emit(name, arg),
                    }),
                    get: (_, command: keyof __EventObj__<any>) => {
                        switch (command) {
                            case 'listen':
                                return (arg: any) => TAURI_API_EVENT.listen(name, arg)
                            case 'once':
                                return (arg: any) => TAURI_API_EVENT.once(name, arg)
                            case 'emit':
                                return (arg: any) => TAURI_API_EVENT.emit(name, arg)
                        }
                    },
                })
            },
        },
    )
}
