<script lang="ts">
    export let symbolId: string
</script>

<button class="rounded-md border border-transparent p-4 ring-inset ring-slate-300 hover:border-slate-300 active:border-transparent active:bg-slate-200 active:ring-1" on:click title={symbolId}>
    <svg class="h-full w-full fill-current">
        <use href="#{symbolId}" />
    </svg>
</button>
