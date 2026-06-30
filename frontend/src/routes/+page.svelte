<script lang="ts">
    interface Block {
        index: number;
        data: String;
        prev_hash: String;
        hash: String;
    }

    let blocks: Block[] = $state([]);
    let loading: boolean = $state(false);
    let error: string | null = $state(null);

    let newData: string = $state("");

    async function fetchBlocks(): Promise<void> {
        loading = true;
        error = null;

        try {
            const response = await fetch("http://localhost:8000/blocks");

            if (!response.ok) {
                throw new Error("error while load data");
            }

            blocks = await response.json();
        } catch (e: any) {
            error = e.message;
        } finally {
            loading = false;
        }
    }

    async function postBlock(): Promise<void> {
        try {
            const response = await fetch("http://localhost:8000/blocks", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ data: newData }),
            });

            if (!response.ok) {
                throw new Error("failed to add block");
            }

            newData = "";
            await fetchBlocks();
        } catch (e: any) {
            error = e.message;
        }
    }

    $effect(() => {
        fetchBlocks();
    });
</script>

<h1>Page</h1>

<div>
    <input type="text" bind:value={newData} placeholder="Enter block data" />
    <button onclick={postBlock}>Add Block</button>
</div>

<h3>Blocks</h3>

{#if loading}
    <p>Loading...</p>
{:else if error}
    <p>Error: {error}</p>
{:else}
    <ul>
        {#each blocks as block}
            <li>{block.data} (Hash: {block.hash})</li>
        {/each}
    </ul>
{/if}
