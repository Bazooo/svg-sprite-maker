<script lang="ts">
    export let symbolId: string
    export let active: boolean
</script>

<button
    class="rounded-md border border-transparent p-4 outline-0 ring-inset ring-slate-300 hover:border-slate-300 active:border-transparent active:bg-slate-200 active:ring-1 dark:hover:border-slate-700 dark:active:bg-slate-800 dark:active:ring-slate-700"
    class:ring-violet-500={active}
    class:ring-1={active}
    on:click
    title={symbolId}
>
    <svg class="h-full w-full fill-current">
        <use href="#{symbolId}" />
    </svg>
</button>
