<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import { mdiCreation } from '@mdi/js'

    let hovering = false
    let hovered = 0
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

    listen<string>('sprite-changed', (event) => {
        sprite = event.payload
    })
</script>

<div class="flex flex-col h-full bg-slate-50 text-neutral-800" role="main">
    <main class="grow relative">
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
        {/if}
    </main>
    <footer class="flex">

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