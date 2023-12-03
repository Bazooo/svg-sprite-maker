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
    <div class="flex shrink-0 items-center border-b border-slate-300 bg-slate-100 dark:bg-slate-900 dark:border-slate-700 px-2 py-1">
        <span class="text">Settings</span>
        <span class="grow" />
        <button class="h-full rounded p-1 hover:bg-slate-300 active:bg-slate-200" on:click={closeSettings}>
            <svg class="h-4 w-4" viewBox="0 0 24 24">
                <path d={mdiClose} />
            </svg>
        </button>
    </div>

    <span>dark mode</span>
    <input type="checkbox" class="w-full rounded bg-slate-200 px-2 py-1 outline-0" checked={$applicationSettings.darkMode} on:change={toggleDarkMode} />

    <span>Editor path:</span>
    <input type="text" class="w-full rounded bg-slate-200 px-2 py-1 outline-0" value={$applicationSettings.editorPath ?? ''} readonly />
    <button class="w-full rounded border border-transparent p-1 text-blue-500 hover:border-blue-500" on:click={setEditorPath}>
        <svg class="h-4 w-4" viewBox="0 0 24 24">
            <path d={mdiApplicationEditOutline} />
        </svg>
    </button>
</aside>
