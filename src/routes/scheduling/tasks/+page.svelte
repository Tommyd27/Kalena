<script>
    import { invoke } from "@tauri-apps/api/tauri";


    // @ts-nocheck

    
    async function fetchTasks() {
        return await invoke("fetch_tasks");
    }

    let tasks = fetchTasks();
</script>
<h1> on tasks</h1>
{#await tasks}
	<p>...fetching tasks</p>
{:then tasks}
	<p>{tasks}</p>
{:catch error}
	<p style="color: red">Failed to Fetch Tasks: {error}</p>
    <button on:click = {fetchTasks}>Refetch</button>
{/await}