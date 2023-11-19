<script lang="ts">
    import { mdiCreation } from '@mdi/js'
    import { listen } from '@tauri-apps/api/event'
    import {onMount} from "svelte";

    let hovering = false
    let hovered = 0

    onMount(async () => {
        const unlistenHover = await listen('tauri://file-drop-hover', () => {
            hovering = true
        })

        const unlistenHoverStop = await listen('files-hover-stopped', () => {
            hovering = false
        })

        const unlistenFilesHovered = await listen<number>('files-hovered', (event) => {
            hovered = event.payload
        })

        return () => {
            unlistenHover()
            unlistenHoverStop()
            unlistenFilesHovered()
        }
    })
</script>

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
