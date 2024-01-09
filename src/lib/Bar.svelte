<script lang="ts">
    export let currentDir: string = "ERROR: no path assigned";
    export let showHidden: boolean= false;
</script>

<div id="bar">
    <p>{currentDir}</p>
    {#if showHidden === true}
        <p>[H]</p>
    {/if}
</div>

<style>
    #bar {
        background-color: var(--crust);
        width: 100%;
        display: flex;
        justify-content: space-between;
        & p {
            padding: 0.25rem 1ch;
        }
    }
</style>
