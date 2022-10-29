<script>
	export let name;
	import KalenaIcon from "./components/KalenaIcon.svelte";
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Vist the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	<p> why the fuck doesnb't this bitch as sowrk asdfgsdepghsaioldehg\osp'eidghsdogij</p>
	<KalenaIcon/>
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
</style>