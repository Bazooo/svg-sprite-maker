<script lang="ts">
    export let symbolId: string
</script>

<button class="p-4 border border-transparent rounded-md hover:border-slate-300 active:border-transparent active:bg-slate-200 ring-inset ring-slate-300 active:ring-1"
        on:click
        title={symbolId}>
    <svg class="fill-current w-full h-full">
        <use href="#{symbolId}" />
    </svg>
</button>
