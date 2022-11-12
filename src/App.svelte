<script>
	export let name;
	import KalenaIcon from "./components/KalenaIcon.svelte";
    import GoodMorning from "./Good Morning.svelte";
	import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from "@tauri-apps/api/tauri";



	let screen = 0;

	async function checkIfNeedDate()
	{
		if (!await invoke('need_date'))
		{
			screen = 1
		}

	}
	checkIfNeedDate()

	
</script>



<div class = "topBar">
	<KalenaIcon/>
</div>



<main>
	{#if screen == 0}
		<GoodMorning></GoodMorning>
	{/if}
</main>



<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}

	.topBar
	{
		position: absolute;
	}

</style>