<script lang="ts">
    import { mdiApplicationEditOutline, mdiClose, mdiCreation } from '@mdi/js'
    import { applicationSettings, settingsWindowOpen } from '../../store'
    import { dialog, invoke } from '@tauri-apps/api'
    import TransparentGrid from '../TransparentGrid.svelte'

    const setEditorPath = async () => {
        const path = await dialog.open({
            title: 'Set editor path',
            multiple: false,
            directory: false,
            filters: [{ name: 'Executable', extensions: ['exe'] }],
        })

        if (path) {
            await invoke('set_editor_path', { path })
        }
    }

    const toggleDarkMode = async () => {
        await invoke('set_dark_mode', { enabled: !$applicationSettings?.darkMode })
    }

    const toggleAutoSave = async () => {
        await invoke('set_auto_save', { enabled: !$applicationSettings?.autoSaveEnabled })
    }

    const setTransparencyGridColor = (isFirst: boolean) => async (event: InputEvent) => {
        const target = event.currentTarget as HTMLInputElement

        const payload = isFirst
            ? {
                  g1: [target.value, $applicationSettings?.transparencyGridColor1?.[1] ?? 0.1],
                  g2: $applicationSettings?.transparencyGridColor2,
              }
            : {
                  g1: $applicationSettings?.transparencyGridColor1,
                  g2: [target.value, $applicationSettings?.transparencyGridColor2?.[1] ?? 0.1],
              }

        await invoke('set_transparency_grid_colors', payload)
    }

    const setOpacityGridColor = (isFirst: boolean) => async (event: InputEvent) => {
        const target = event.currentTarget as HTMLInputElement

        const payload = isFirst
            ? {
                  g1: [$applicationSettings?.transparencyGridColor1?.[0] ?? 'black', target.valueAsNumber],
                  g2: $applicationSettings?.transparencyGridColor2,
              }
            : {
                  g1: $applicationSettings?.transparencyGridColor1,
                  g2: [$applicationSettings?.transparencyGridColor2?.[0] ?? 'transparent', target.valueAsNumber],
              }

        await invoke('set_transparency_grid_colors', payload)
    }

    const closeSettings = () => {
        settingsWindowOpen.set(false)
    }
</script>

<aside class="flex h-1/2 shrink-0 flex-col overflow-y-auto border-t border-slate-300 bg-slate-200 dark:border-slate-700 dark:bg-slate-800">
    <div class="flex shrink-0 items-center border-b border-slate-300 bg-slate-100 px-2 py-1 dark:border-slate-700 dark:bg-slate-900">
        <span>Settings</span>
        <span class="grow" />
        <button class="h-full rounded p-1 hover:bg-slate-300 active:bg-slate-200 hover:dark:bg-slate-700 active:dark:bg-slate-800" on:click={closeSettings}>
            <svg class="h-4 w-4" viewBox="0 0 24 24">
                <path d={mdiClose} />
            </svg>
        </button>
    </div>

    <div class="flex grow flex-col gap-2 overflow-y-auto px-2 py-2">
        <label class="settings-section">
            <span>Dark mode</span>
            <input type="checkbox" checked={$applicationSettings.darkMode} on:change={toggleDarkMode} />
        </label>

        <label class="settings-section">
            <span>Auto-save</span>
            <input type="checkbox" checked={$applicationSettings.autoSaveEnabled} on:change={toggleAutoSave} />
        </label>

        <div class="settings-section">
            <span>Editor path</span>
            <div class="flex items-center gap-1 rounded border border-slate-400 p-1">
                <input type="text" class="w-full grow rounded bg-transparent outline-0" value={$applicationSettings.editorPath ?? ''} title={$applicationSettings.editorPath ?? ''} readonly />
                <button class="rounded p-1 text-neutral-400 hover:bg-slate-400 hover:text-neutral-200 active:bg-slate-500 active:text-neutral-100 hover:dark:bg-slate-600 active:dark:bg-slate-500" on:click={setEditorPath}>
                    <svg class="h-4 w-4" viewBox="0 0 24 24">
                        <path d={mdiApplicationEditOutline} />
                    </svg>
                </button>
            </div>
        </div>

        <div class="settings-section">
            <span>Transparency Grid</span>
            <div class="flex gap-2">
                <div class="flex flex-col gap-2">
                    <label>
                        <span class="whitespace-nowrap rounded bg-slate-400 px-2 py-1 text-xs text-white">Color 1</span>
                        <input type="text" class="w-full grow rounded bg-transparent outline-0" value={$applicationSettings.transparencyGridColor1?.[0] ?? 'black'} on:change={setTransparencyGridColor(true)} />
                    </label>
                    <label>
                        <span class="whitespace-nowrap rounded bg-slate-400 px-2 py-1 text-xs text-white">Opacity 1</span>
                        <input type="number" min="0" max="1" step="0.1" class="w-full grow rounded bg-transparent outline-0" value={$applicationSettings.transparencyGridColor1?.[1].toPrecision(1) ?? 1} on:change={setOpacityGridColor(true)} />
                    </label>
                </div>
                <div class="relative aspect-square h-full shrink-0 overflow-hidden rounded border border-slate-400">
                    <TransparentGrid />
                    <svg class="absolute z-10 h-full w-full fill-current" viewBox="0 0 24 24">
                        <path d={mdiCreation} />
                    </svg>
                </div>
                <div class="flex flex-col gap-2">
                    <label>
                        <span class="whitespace-nowrap rounded bg-slate-400 px-2 py-1 text-xs text-white">Color 2</span>
                        <input type="text" class="w-full grow rounded bg-transparent outline-0" value={$applicationSettings.transparencyGridColor2?.[0] ?? 'transparent'} on:change={setTransparencyGridColor(false)} />
                    </label>
                    <label>
                        <span class="whitespace-nowrap rounded bg-slate-400 px-2 py-1 text-xs text-white">Opacity 1</span>
                        <input type="number" min="0" max="1" step="0.1" class="w-full grow rounded bg-transparent outline-0" value={$applicationSettings.transparencyGridColor2?.[1].toPrecision(1) ?? 0} on:change={setOpacityGridColor(false)} />
                    </label>
                </div>
            </div>
        </div>
    </div>
</aside>

<style>
    .settings-section {
        @apply flex flex-col gap-2 rounded p-2 hover:bg-slate-300 hover:dark:bg-slate-700;

        & label {
            @apply flex items-center gap-1 rounded  border border-slate-400 p-1 hover:border-slate-500;
        }
    }

    .settings-section > span {
        @apply text-xs font-bold uppercase;
    }
</style>
