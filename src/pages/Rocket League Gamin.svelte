<script>
	import { invoke } from "@tauri-apps/api/tauri"
	import Player from "./components/Player.svelte"
	let players = []
	let playersOutput = []
	let rows = ["Name", "MMR", "Respects", "Mechanical", "Speed"]
	let statsToTrack = ["Respects", "Mechanical", "Speed"];
	async function startListening()
	{
		players = await invoke('fetch_players');
		playersOutput = new Array(players.length).fill(Object);
		/*playersOutput.forEach(x =>{
			statsToTrack.forEach(stat =>{
				x[stat] = stat
			})
		})*/
		console.log(playersOutput)
	}
	$: playerNumbers = players.length + 1
</script>

<button on:click={startListening}>PRESS ME PRESS ME PRESSES ME NOM NUM NUM</button>

<div class = "tableGrid" style = "--playerNumbers: {playerNumbers}">
	{#each rows as row, i}
		<h1 style = "grid-row-start: {i+1}; grid-row-end: {i+1}">{row}</h1>
		<!--<h1>{row}</h1>-->
	{/each}
	{#each players as player, i}
		<h1 style = "grid-row-start: 1; grid-row-end:1; grid-column-start:{i + 2}; grid-column-end:{i + 2}">{player.name}</h1>
		<h1 style = "grid-row-start: 2; grid-row-end:2; grid-column-start:{i + 2}; grid-column-end:{i + 2}">{Math.round(player.mmr)}</h1>
		<!--<h1 style = "grid-row-start: 3; grid-row-end:3; grid-column-start:{i + 2}; grid-column-end:{i + 2}">{player.id}</h1>-->
		{#each statsToTrack as row, j}
			<input type=range min=0 max = 10 bind:value={playersOutput[j][row]} style = "grid-row-start:{j + 3}; grid-row-finish:{j + 3}; grid-column-start:{i + 2}; grid-column-end:{i + 2}">
		{/each}

	{/each}
		
</div>


<style>
	.tableGrid{
		display: grid;
		position: absolute;
		gap: 1px 1px;
		grid-template-rows: repeat(5, 1fr);
		border: 1px solid #000;
		grid-template-columns: repeat(var(--playerNumbers), 1fr);
		background-color: black;
	}
	h1{
		font-size: 20px;  
		grid-column: 0; 
		padding: 0px 10px 0px 0px;
		margin: 0px 0px 0px 0px; 
		border: 0px;
		background-color: white;
		text-align: left;
	}
	.player{
		grid-row-start: 1;
		grid-row-end: 6;
	}
	input{
		
	}
</style>



