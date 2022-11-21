<script>
	export let name = "";
	export let mmr = 0;
	export let id = "-1";
	export let statsToTrackStr = ["Respects", "Mechanical", "Speed"];
	let statsToTrackVals = new Array(statsToTrackStr.length);
	let notes = "";
</script>

<h1>{name}</h1>
<h1>{mmr}</h1>
<h1>{id}</h1>

{#each statsToTrackStr as stat, i}
	<input type = number bind:value = {statsToTrackVals[i]}>
{/each}

<input bind:value = {notes}>

<style>
	h1{
	font-size: 20px;  
	padding: 0px; 
	border: 0px;
}
</style>
