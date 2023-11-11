<script lang="ts">
    import { listen } from "@tauri-apps/api/event"

    let hovered = 0

    listen('file-drop-cancelled', () => {
        console.log('file-drop-cancelled')
    })

    listen<number>('files-hovered', (event) => {
        hovered = event.payload
    })
</script>

<main>
    <span>hovering: {hovered}</span>
</main>

<style>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>