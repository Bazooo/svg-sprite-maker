<script lang="ts">
    import SideMenu from './components/SideMenu/SideMenu.svelte'
    import SymbolButton from './components/SymbolButton.svelte'
    import FileHoverIndicator from './components/FileHoverIndicator.svelte'
    import { onMount } from 'svelte'
    import { activeSymbolIds, applicationSettings, openedFooterWindow, sprite, symbolIds } from './store'
    import Footer from './components/Footer/Footer.svelte'
    import Settings from './components/FooterWindows/Settings.svelte'
    import { handleShortcut } from './shortcuts'
    import Shortcuts from './components/FooterWindows/Shortcuts.svelte'
    import ToolBar from './components/ToolBar/ToolBar.svelte'
    import { commands, events } from './types/bindings'

    const toggleActiveSymbolId = (id: string) => (event: MouseEvent) => {
        if (event.shiftKey) {
            activeSymbolIds.update((symbols) => {
                return symbols.includes(id) ? symbols.filter((symbol) => symbol !== id) : [...symbols, id]
            })
            return
        }

        activeSymbolIds.set([id])
    }

    onMount(() => {
        let unlistenSpriteChanged: () => unknown
        let unlistenEditorNotSet: () => unknown
        let unlistenSettingsChanged: () => unknown

        const mount = async () => {
            const currentApplicationSettings = await commands.getAppSettings()
            if (currentApplicationSettings) {
                applicationSettings.set(currentApplicationSettings)
            }

            unlistenSpriteChanged = await events.spriteChangedEvent.listen((event) => {
                symbolIds.set(event.payload.ids)
                sprite.set(event.payload.sprite)
            })

            unlistenEditorNotSet = await events.editorNotSetEvent.listen(() => {
                openedFooterWindow.set('settings')
            })

            unlistenSettingsChanged = await events.settingsChangedEvent.listen((event) => {
                applicationSettings.set(event.payload)
            })
        }

        void mount()

        return () => {
            unlistenSpriteChanged()
            unlistenEditorNotSet()
            unlistenSettingsChanged()
        }
    })
</script>

<div class="flex h-full flex-col border-t border-slate-300 bg-slate-50 text-neutral-800 dark:bg-slate-950 dark:text-neutral-200" role="main">
    <div class="relative flex grow overflow-hidden">
        <FileHoverIndicator />
        <div class="flex grow flex-col">
            {#if $sprite}
                <ToolBar />
            {/if}
            <main class="grow overflow-y-auto">
                {#if !$sprite}
                    <div class="flex h-full w-full items-center justify-center">
                        <span class="select-none">Drop svg file(s)</span>
                    </div>
                {:else if $symbolIds.length === 0}
                    <div class="flex h-full w-full items-center justify-center">
                        <span class="select-none">No symbols</span>
                    </div>
                {:else}
                    <div class="symbols-grid grid justify-center gap-4 p-3">
                        {#each $symbolIds as symbolId (symbolId)}
                            <SymbolButton {symbolId} active={$activeSymbolIds.includes(symbolId)} on:click={toggleActiveSymbolId(symbolId)} />
                        {/each}
                    </div>
                {/if}
            </main>

            {#if $openedFooterWindow === 'settings'}
                <Settings />
            {:else if $openedFooterWindow === 'shortcuts'}
                <Shortcuts />
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

<svelte:window on:keydown={handleShortcut} />

<style>
    .symbols-grid {
        grid-template-columns: repeat(auto-fill, 4.25rem);
        grid-auto-rows: 4.25rem;
    }
</style>
