<script>
	import { appWindow } from "@tauri-apps/api/window";
	


	async function startListening()
	{
		await appWindow.emit("get_rocket_league_players");
	}
</script>


