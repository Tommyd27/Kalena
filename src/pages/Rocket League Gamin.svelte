<script>
	import { invoke } from "@tauri-apps/api/tauri"
	import Player from "./components/Player.svelte"
	let players = []
	let rows = ["Name", "MMR", "ID", "Respects", "Mechanical", "Speed"]
	let statsToTrack = ["Respects", "Mechanical", "Speed"];
	async function startListening()
	{
		players = await invoke('fetch_players');
		players.forEach(element => {
			console.log(element.name)
			console.log(element["name"])
		});
		console.log(players)
	}
	let playerNumbers = $players.length + 1
</script>

<button on:click={startListening}>PRESS ME PRESS ME PRESSES ME NOM NUM NUM</button>

<div class = "tableGrid" style = "--playerNumbers: {$playerNumbers}">
	{#each rows as row, i}
		<h1 style = "grid-row: {i + 1}">{row}</h1>
		<!--<h1>{row}</h1>-->
	{/each}
	{#each players as player}
		<Player style = "grid-column: 2;" name = {player.name} mmr = {player.mmr} id = {player.id}></Player>
	{/each}
</div>


<style>
	.tableGrid{
		display: grid;
		position: absolute;
		gap: 0px 0px;
		grid-template-rows: repeat(6, minmax(15px, 0.5fr));
		border: 1px solid #000;
		grid-template-columns: repeat(var(--playerNumbers), 1fr)
	}
	h1{
		font-size: 20px;  
		grid-column: 0; 
		padding: 0px; 
		border: 0px;
	}
</style>




