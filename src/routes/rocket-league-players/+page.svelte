<script>
	import { invoke } from "@tauri-apps/api/tauri"
	class Player {
		constructor(id, name, generalStats) {
			this.id = id
			this.name = name
			this.generalStats = generalStats
		}
	}
	let players = []
	let playersOutput = []
	let rows = ["Name", "MMR"]
	let statsToTrack = ["Respects", "Mechanical", "Speed"];
	async function startListening() {
		players = await invoke('fetch_players');
		playersOutput = new Array(players.length); 
		
		for (let i = 0; i < players.length; i++) {
			playersOutput[players[i].id] = new Object;
			playersOutput[players[i].id].name = players[i].name
			playersOutput[players[i].id].mmr = players[i].mmr
		}
	}
	function output()
	{
		console.log(playersOutput)
		console.log(Object.keys(playersOutput))
		console.log(Object.keys(playersOutput["C"]))
	}
	async function sendStats(){
		let playersInfo = []
		for (const [key, value] of Object.entries(playersOutput))
		{
			let id = key
			let name = value["name"]
			delete value.name
			let generalStats = []
			for (const [kKey, vValue] of Object.entries(value))
			{
				console.log(kKey, vValue)
				generalStats.push([kKey, vValue])
				
			}
			playersInfo.push(new Player (id, name, generalStats))
		}
		console.log(playersInfo);
		console.log(await invoke('insert_players', {playersInfo}));
		playersOutput = []
		players = []
	}
	$: playerNumbers = players.length + 1
</script>

<button on:click={startListening}>PRESS ME PRESS ME PRESSES ME NOM NUM NUM</button>

<div class = "tableGrid" style = "--playerNumbers: {playerNumbers}">
	{#each rows as row, i}
		<h1 style = "grid-row-start: {i+1}; grid-row-end: {i+1}">{row}</h1>
	{/each}

	<h1 style = "grid-row-start:3; grid-row-end:3">Notes</h1>
	{#each statsToTrack as row, i}
		<h1 style = "grid-row-start: {2 * i + 4}; grid-row-end: {2 * i + 4}">{row}</h1>
		<h1 style = "grid-row-start: {2 * i + 5}; grid-row-end: {2 * i + 5}; background-color: red">{row}</h1>
	{/each}

	{#each players as player, i}
		<h1 style = "grid-row-start: 1; grid-row-end:1; grid-column-start:{i + 2}; grid-column-end:{i + 2}">{player.name}</h1>
		<h1 style = "grid-row-start: 2; grid-row-end:2; grid-column-start:{i + 2}; grid-column-end:{i + 2}">{Math.round(player.mmr)}</h1>
		<input bind:value={playersOutput[player.id]["notes"]}>
		<!--<h1 style = "grid-row-start: 3; grid-row-end:3; grid-column-start:{i + 2}; grid-column-end:{i + 2}">{player.id}</h1>-->
		{#each statsToTrack as row, j}
			<input type=range min=0 max = 10 bind:value={playersOutput[player.id][row]} style = "grid-row: 1; grid-row-start:{2 * j + 4}; grid-row-finish:{2 * j + 4}; grid-column-start:{i + 2}; grid-column-end:{i + 2}">
			<input type=range min=0 max = 10 bind:value={playersOutput[player.id]["e" + row]} style = "grid-row: 2;grid-row-start:{2 * j + 5}; grid-row-finish:{2 * j + 5}; grid-column-start:{i + 2}; grid-column-end:{i + 2}">	
		{/each}

	{/each}
		
</div>

<button on:click={output}>Cock</button>
<button on:click={sendStats}>Send Stats</button>


<style>
	.tableGrid{
		display: grid;
		position: absolute;
		gap: 1px 1px;
		grid-template-rows: 1fr 1fr 2fr repeat(6, 1fr);
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
	input{
		
	}
</style>
