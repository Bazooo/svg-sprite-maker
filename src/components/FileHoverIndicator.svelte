<script lang="ts">
    import { mdiCreation } from '@mdi/js'
    import { listen } from '@tauri-apps/api/event'
    import { onMount } from 'svelte'
    import { events } from '../types/bindings'

    let hovering = false
    let hovered = 0

    onMount(() => {
        let unlistenHover: () => unknown
        let unlistenHoverStop: () => unknown
        let unlistenFilesHovered: () => unknown

        const mount = async () => {
            unlistenHover = await listen('tauri://file-drop-hover', () => {
                hovering = true
            })

            unlistenHoverStop = await events.filesHoverStoppedEvent.listen(() => {
                hovering = false
            })

            unlistenFilesHovered = await events.filesHoveredEvent.listen((event) => {
                hovered = event.payload
            })
        }

        void mount()

        return () => {
            unlistenHover()
            unlistenHoverStop()
            unlistenFilesHovered()
        }
    })
</script>

{#if hovering && hovered}
    <div class="absolute left-0 top-0 z-[9999] flex h-full w-full select-none items-center justify-center gap-x-2 rounded-3xl border-8 border-dashed border-current bg-slate-50/90 dark:bg-slate-950/90">
        <svg class="h-10 w-10 fill-current" viewBox="0 0 24 24">
            <path d={mdiCreation} />
        </svg>
        <span class="text-3xl font-bold">
            {hovered}
        </span>
    </div>
{/if}
