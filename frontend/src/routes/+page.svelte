<script lang="ts">
    interface Block {
        index: number;
        data: string;
        prev_hash: string;
        hash: string;
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
        } catch (e: unknown) {
            if (e instanceof Error) {
                error = e.message;
            }
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

            const result = await response.json();

            if (!response.ok) {
                throw new Error(result.error || "Failed to add block");
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

<div class="p-5">
    <h1>Page</h1>

    <div>
        <input
            type="text"
            bind:value={newData}
            placeholder="Enter block data"
            class="border border-gray-300 rounded px-2 py-1 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
            class="p-2 bg-sky-300 h-10 w-24 rounded-sm disabled:opacity-50"
            onclick={postBlock}
            disabled={newData.trim().length === 0}
        >
            Add Block
        </button>
    </div>

    <h3 class="underline">Blocks</h3>
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
</div>
