<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import type { SpriteChangedEvent } from './types/spriteChangedEvent'
    import SideMenu from './components/SideMenu/SideMenu.svelte'
    import SymbolButton from './components/SymbolButton.svelte'
    import FileHoverIndicator from './components/FileHoverIndicator.svelte'
    import { onMount } from 'svelte'
    import { activeSymbolId, applicationSettings, settingsWindowOpen, sprite, symbolIds } from './store'
    import { dialog, invoke } from '@tauri-apps/api'
    import type { ApplicationSettings } from './types/applicationSettings'
    import Footer from './components/Footer/Footer.svelte'
    import Settings from './components/Settings/Settings.svelte'

    const setActiveSymbolId = (id: string) => () => {
        activeSymbolId.set(id)
    }

    const changeSaveFilePath = async () => {
        const path = await dialog.save({
            defaultPath: 'sprite.svg',
            filters: [{ name: 'SVG', extensions: ['svg'] }],
        })
        await invoke('set_save_file_path', { path })
    }

    const shortcutHandler = async (event: KeyboardEvent) => {
        if (!event.ctrlKey) {
            return
        }

        if (event.key === 's') {
            if (event.shiftKey) {
                await changeSaveFilePath()
            } else {
                await invoke('save')
            }
        }
    }

    onMount(async () => {
        const currentApplicationSettings = await invoke<ApplicationSettings>('get_app_settings')
        console.log(currentApplicationSettings)
        applicationSettings.set(currentApplicationSettings)

        const unlistenSpriteChanged = await listen<SpriteChangedEvent>('sprite-changed', (event) => {
            symbolIds.set(event.payload.ids)
            sprite.set(event.payload.sprite)
        })

        const unlistenSaveFileNotSet = await listen('save-file-not-set', changeSaveFilePath)

        const unlistenEditorNotSet = await listen('editor-not-set', () => {
            settingsWindowOpen.set(true)
        })

        const unlistenSettingsChanged = await listen<ApplicationSettings>('settings-changed', (event) => {
            applicationSettings.set(event.payload)
        })

        return () => {
            unlistenSpriteChanged()
            unlistenSaveFileNotSet()
            unlistenEditorNotSet()
            unlistenSettingsChanged()
        }
    })
</script>

<div class="flex h-full flex-col bg-slate-50 dark:bg-slate-950 text-neutral-800 dark:text-neutral-200" role="main">
    <div class="relative flex grow overflow-hidden">
        <FileHoverIndicator />

        <div class="flex grow flex-col">
            <main class="grow overflow-y-auto">
                {#if !$sprite}
                    <div class="flex h-full w-full items-center justify-center">
                        <span class="select-none">Drop svg file(s)</span>
                    </div>
                {:else if $symbolIds.length > 0}
                    <div class="symbols-grid grid justify-center gap-4 p-3">
                        {#each $symbolIds as symbolId (symbolId)}
                            <SymbolButton {symbolId} on:click={setActiveSymbolId(symbolId)} />
                        {/each}
                    </div>
                {/if}
            </main>

            {#if $settingsWindowOpen}
                <Settings />
            {/if}
        </div>

        {#if $sprite}
            <SideMenu />
        {/if}
    </div>
    <Footer />
</div>

<div class="hidden">
    {#if $sprite}
        {@html $sprite}
    {/if}
</div>

<svelte:window on:keydown={shortcutHandler} />

<style>
    .symbols-grid {
        grid-template-columns: repeat(auto-fill, 4.25rem);
        grid-auto-rows: 4.25rem;
    }
</style>
