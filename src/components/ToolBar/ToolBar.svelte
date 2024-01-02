<script lang="ts">
    import { symbolIds } from '../../store'
    import type { KeyboardEventHandler } from 'svelte/elements'
    import { mdiMagnify } from '@mdi/js'
    import { commands } from '../../types/bindings'

    const searchIds: KeyboardEventHandler<HTMLInputElement> = async (event) => {
        const query = event.currentTarget.value

        const ids = await commands.searchSymbolsById(query)
        symbolIds.set(ids)
    }
</script>

<aside class="flex flex-col justify-center gap-1 p-4">
    <label class="flex items-center gap-2 rounded border border-slate-300 p-1 dark:border-slate-700">
        <svg class="h-5 w-5 fill-current text-slate-600" viewBox="0 0 24 24">
            <path d={mdiMagnify} />
        </svg>
        <input type="text" class="bg-transparent placeholder-slate-300 outline-0" placeholder="Search ID..." on:input={searchIds} />
    </label>
</aside>
