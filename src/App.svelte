<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import type { SpriteChangedEvent } from './types/spriteChangedEvent'
    import SideMenu from './components/sidemenu/SideMenu.svelte'
    import SymbolButton from './components/SymbolButton.svelte'
    import FileHoverIndicator from './components/FileHoverIndicator.svelte'
    import { onMount } from 'svelte'
    import { activeSymbolId, sprite, symbolIds } from './store'
    import { dialog, invoke } from '@tauri-apps/api'

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
        const unlistenSpriteChanged = await listen<SpriteChangedEvent>('sprite-changed', (event) => {
            symbolIds.set(event.payload.ids)
            sprite.set(event.payload.sprite)
            console.log($symbolIds)
        })

        const unlistenSaveFileNotSet = await listen('save-file-not-set', changeSaveFilePath)

        return () => {
            unlistenSpriteChanged()
            unlistenSaveFileNotSet()
        }
    })
</script>

<div class="flex h-full flex-col bg-slate-50 text-neutral-800" role="main">
    <div class="relative flex grow overflow-hidden">
        <FileHoverIndicator />
        <main class="grow overflow-y-auto">
            {#if !$sprite}
                <div class="flex h-full w-full items-center justify-center">
                    <span class="select-none">Drop svg file(s)</span>
                </div>
            {:else if $symbolIds.length > 0}
                <div class="symbols-grid grid justify-center gap-4 p-3">
                    {#each $symbolIds as symbolId(symbolId)}
                        <SymbolButton {symbolId} on:click={setActiveSymbolId(symbolId)} />
                    {/each}
                </div>
            {/if}
        </main>

        {#if $sprite}
            <SideMenu />
        {/if}
    </div>
    <footer class="flex bg-slate-200">
        <kbd>Ctrl + S</kbd>
    </footer>
</div>

<div class="hidden">
    {#if $sprite}
        {@html $sprite}
    {/if}
</div>

<svelte:window on:keydown={shortcutHandler} />

<style>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;

    :global(body),
    :global(html),
    :global(#app) {
        width: 100%;
        height: 100%;
    }

    .symbols-grid {
        grid-template-columns: repeat(auto-fill, 4.25rem);
        grid-auto-rows: 4.25rem;
    }
</style>
