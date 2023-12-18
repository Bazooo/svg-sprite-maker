<script>
    import FooterWindow from './FooterWindow.svelte'

    const shortcuts = [
        {
            name: 'Save',
            keys: ['Ctrl', 'S'],
        },
        {
            name: 'Save to another file',
            keys: ['Ctrl', 'Shift', 'S'],
        },
        {
            name: 'Remove all symbols and reset',
            keys: ['Ctrl', 'N'],
        },
    ]
</script>

<FooterWindow title="Shortcuts">
    <div class="flex flex-wrap overflow-y-auto py-2">
        {#each shortcuts as shortcut}
            <div class="flex w-1/2 justify-between px-2 py-1 hover:bg-slate-300 dark:hover:bg-slate-800">
                <span>{shortcut.name}</span>
                <div class="flex items-center gap-1">
                    {#each shortcut.keys as key}
                        <kbd class="rounded border border-slate-400 p-1 text-xs leading-none dark:border-slate-600">{key}</kbd>
                    {/each}
                </div>
            </div>
        {/each}
    </div>
</FooterWindow>
