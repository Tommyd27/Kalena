<script>
	export let name;
	import KalenaIcon from "./components/KalenaIcon.svelte";
    import GoodMorning from "./Good Morning.svelte";
</script>



<div class = "topBar">
	<KalenaIcon/>
</div>



<main>
	<GoodMorning></GoodMorning>
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