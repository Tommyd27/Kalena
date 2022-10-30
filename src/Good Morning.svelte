

<script>
	import { invoke } from "@tauri-apps/api/tauri"
	let time = "";
	let greetMsg = "";

	async function sendInput()
	{
		greetMsg = await invoke('hello', {time});
	}
</script>


<h1>
	Good Morning
</h1>
<h2>Bitch</h2>
<label for="appt">What time did you wake up:</label>

<input type="time" id="appt" name="appt" required bind:value= {time}>

<button type=submit on:click={sendInput}>Enter.</button> 

<p>{time}</p>
<p>{greetMsg}</p>



<style>
	h1
	{
		margin-bottom: 0;
	}
	h2 {
		font-size: 100px;
		margin-top: -20px;
		background: linear-gradient(to right, #ef5350, #f48fb1, #7e57c2, #2196f3, #26c6da, #43a047, #eeff41, #f9a825, #ff5722);
  		-webkit-background-clip: text;
		background-clip: text;
  		-webkit-text-fill-color: transparent;
	}
</style>