<script>
	import KalenaIcon from "./components/KalenaIcon.svelte";
    import GoodMorning from "./Good Morning.svelte";
	import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from "@tauri-apps/api/tauri";
	import { currentPage } from "./stores.js";


	let debugUI = true;
	async function checkIfNeedDate()
	{
		if (!await invoke('need_date'))
		{
			currentPage.set(1);
		}

	}
	checkIfNeedDate()

	
</script>



<div class = "topBar">
	<KalenaIcon/>
</div>

<div class = "topRight">
	{#if debugUI}
		<input bind:value={$currentPage} size=1>
	{/if}
</div>



<main>
	{#if $currentPage == 0}
		<GoodMorning></GoodMorning>
	{/if}
	
</main>



<style>
	main 
	{
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;	
	}

	h1 
	{
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) 
	{
		main {
			max-width: none;
		}
	}

	
	.topBar
	{
		position: absolute;
	}

	.topRight
	{
		position: absolute; 
		top: 0; 
		right: 0
	}

</style>