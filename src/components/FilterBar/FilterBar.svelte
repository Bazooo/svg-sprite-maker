<script lang="ts">
    import { symbolIds } from '../../store'
    import type { KeyboardEventHandler } from 'svelte/elements'
    import { invoke } from '@tauri-apps/api'
    import { mdiMagnify } from '@mdi/js'

    const searchIds: KeyboardEventHandler<HTMLInputElement> = async (event) => {
        const query = event.currentTarget.value

        const ids = await invoke<string[]>('search_symbols_by_id', { query })
        symbolIds.set(ids)
    }
</script>

<aside class="flex justify-between gap-2 border-b border-slate-300 p-1 dark:border-slate-700">
    <div></div>
    <label class="flex items-center gap-2">
        <svg class="h-5 w-5 fill-current text-slate-600" viewBox="0 0 24 24">
            <path d={mdiMagnify} />
        </svg>
        <input type="text" class="bg-transparent placeholder-slate-300 outline-0" placeholder="Search by ID" on:input={searchIds} />
    </label>
</aside>
