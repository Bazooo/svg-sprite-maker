<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import { mdiCreation } from '@mdi/js'
    import type { SpriteChangedEvent } from './types/spriteChangedEvent'
    import SideMenu from './components/sidemenu/SideMenu.svelte'

    let hovering = false
    let hovered = 0
    let symbolIds: string[] = []
    let sprite: string | undefined
    let activeSymbolId: string | undefined

    listen('tauri://file-drop-hover', () => {
        hovering = true
    })

    listen('files-hover-stopped', () => {
        hovering = false
    })

    listen<number>('files-hovered', (event) => {
        hovered = event.payload
    })

    listen<SpriteChangedEvent>('sprite-changed', (event) => {
        symbolIds = event.payload.ids
        sprite = event.payload.sprite
    })

    const setActiveSymbolId = (id: string) => () => {
        activeSymbolId = id
    }
</script>

<div class="flex flex-col h-full bg-slate-50 text-neutral-800" role="main">
    <div class="flex grow relative overflow-hidden">
        {#if hovering && hovered}
            <div class="w-full h-full flex justify-center items-center gap-x-2 absolute z-[9999] top-0 left-0 bg-slate-50/90 border-8 border-current border-dashed rounded-3xl select-none">
                <svg class="fill-current h-10 w-10" viewBox="0 0 24 24">
                    <path d={mdiCreation} />
                </svg>
                <span class="text-3xl font-bold">
                    {hovered}
                </span>
            </div>
        {/if}
        <main class="grow overflow-y-auto">
            {#if !sprite}
                <div class="w-full h-full flex justify-center items-center">
                    <span class="select-none">Drop svg file(s)</span>
                </div>
            {:else if symbolIds.length > 0}
                <div class="symbols-grid grid gap-4 p-3 justify-center">
                    {#each symbolIds as symbolId}
                        <button class="p-4 border border-transparent rounded-md hover:border-slate-300 active:border-transparent active:bg-slate-200 ring-inset ring-slate-300 active:ring-1"
                                on:click={setActiveSymbolId(symbolId)}
                                title={symbolId}>
                            <svg class="fill-current w-full h-full">
                                <use href="#{symbolId}" />
                            </svg>
                        </button>
                    {/each}
                </div>
            {/if}
        </main>

        {#if sprite}
            <SideMenu {activeSymbolId} />
        {/if}
    </div>
    <footer class="flex bg-slate-200">
        <kbd>Ctrl + S</kbd>
    </footer>
</div>

<div class="hidden">
    {#if sprite}
        {@html sprite}
    {/if}
</div>

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