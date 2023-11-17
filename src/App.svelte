<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import { mdiCreation } from '@mdi/js'
    import type { SpriteChangedEvent } from './types/spriteChangedEvent';

    let hovering = false
    let hovered = 0
    let ids: string[] = []
    let sprite: string | undefined

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
        ids = event.payload.ids
        sprite = event.payload.sprite
    })
</script>

<div class="flex flex-col h-full bg-slate-50 text-neutral-800" role="main">
    <main class="grow relative overflow-y-auto">
        {#if hovering && hovered}
            <div class="w-full h-full flex justify-center items-center gap-x-2 absolute top-0 left-0 bg-slate-50/90 border-8 border-current border-dashed rounded-3xl select-none">
                <svg class="fill-current h-10 w-10" viewBox="0 0 24 24">
                    <path d={mdiCreation} />
                </svg>
                <span class="text-3xl font-bold">
                    {hovered}
                </span>
            </div>
        {/if}
        {#if !sprite}
            <div class="w-full h-full flex justify-center items-center">
                <span class="select-none">Drop svg file(s)</span>
            </div>
        {:else if ids.length > 0}
            <div class="flex flex-wrap gap-4 p-3">
                {#each ids as id}
                    <button class="p-4 border border-transparent rounded-md hover:border-slate-300 active:border-transparent active:bg-slate-200 ring-inset ring-slate-300 active:ring-1" title={id}>
                        <svg class="fill-current w-9 h-9">
                            <use href="#{id}" />
                        </svg>
                    </button>
                {/each}
            </div>
        {/if}
    </main>
    <footer class="flex bg-slate-200">
        <span>hello</span>
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
</style>