<script>
	import { invoke } from "@tauri-apps/api/tauri"
	import Player from "./components/Player.svelte"
	let players = []
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
</script>

<button on:click={startListening}>PRESS ME PRESS ME PRESSES ME NOM NUM NUM</button>


{#each players as player}
	<Player name = {player.name} mmr = {player.mmr} id = {player.id}></Player>
{/each}



