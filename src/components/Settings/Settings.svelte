<script lang="ts">
    import { mdiApplicationEditOutline, mdiClose } from '@mdi/js'
    import { applicationSettings, settingsWindowOpen } from '../../store'
    import { dialog, invoke } from '@tauri-apps/api'

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

    <div class="flex grow flex-col gap-2 px-2 py-2 overflow-y-auto">
        <label class="settings-section">
            <span>Dark mode</span>
            <input type="checkbox" checked={$applicationSettings.darkMode} on:change={toggleDarkMode} />
        </label>

        <div class="settings-section">
            <span>Editor path</span>
            <div class="flex items-center rounded border border-slate-400 p-1 gap-1">
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
            <input type="text" />
        </div>
    </div>
</aside>

<style>
    .settings-section {
        @apply flex flex-col gap-2 rounded p-2 hover:bg-slate-300 hover:dark:bg-slate-700;
    }

    .settings-section > span {
        @apply text-xs font-bold uppercase;
    }
</style>
