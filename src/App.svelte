<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import type { SpriteChangedEvent } from './types/spriteChangedEvent'
    import SideMenu from './components/sidemenu/SideMenu.svelte'
    import SymbolButton from './components/SymbolButton.svelte'
    import FileHoverIndicator from './components/FileHoverIndicator.svelte'
    import {onMount} from "svelte";
    import {activeSymbolId, sprite, symbolIds} from "./store";

    const setActiveSymbolId = (id: string) => () => {
        activeSymbolId.set(id)
    }

    onMount(async () => {
        return await listen<SpriteChangedEvent>('sprite-changed', (event) => {
            symbolIds.set(event.payload.ids)
            sprite.set(event.payload.sprite)
        })
    })
</script>

<div class="flex flex-col h-full bg-slate-50 text-neutral-800" role="main">
    <div class="flex grow relative overflow-hidden">
        <FileHoverIndicator />
        <main class="grow overflow-y-auto">
            {#if !$sprite}
                <div class="w-full h-full flex justify-center items-center">
                    <span class="select-none">Drop svg file(s)</span>
                </div>
            {:else if $symbolIds.length > 0}
                <div class="symbols-grid grid gap-4 p-3 justify-center">
                    {#each $symbolIds as symbolId}
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