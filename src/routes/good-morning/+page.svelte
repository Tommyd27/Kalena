<script>
	import { invoke } from "@tauri-apps/api/tauri"
	import Notification from "$lib/Notification.svelte";
	let time = "";
	let notifVisible = false;
	let notifText = ""
	async function sendInput()
	{
		let fetchTimeResponse = false
		if(time != "")
		{
			fetchTimeResponse = await invoke('send_time_wake', {time});
		}
		console.log(fetchTimeResponse)
		if(fetchTimeResponse)
		{
			window.location.href = "../"
		}
		else
		{
			notifVisible = true;
			notifText = "There was an error sending time"
			setTimeout(() => {
				notifVisible = false
			}, 3000);
		}
	}
</script>


<h1>
	Good Morning
</h1>
<h2>Bitch</h2>
<label for="appt">What time did you wake up:</label>

<input type="time" id="appt" name="appt" required bind:value= {time}>

<button type=submit on:click={sendInput}>Enter.</button> 


<Notification visible = {notifVisible} notificationText = {notifText}/>



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