<script>
	import { invoke } from "@tauri-apps/api/tauri"
    async function checkIfNeedDate()
	{
		let needDate = await invoke('need_date')
		console.log("hello")
		console.log(needDate)
		if (needDate)
		{
			window.location.href = "/good-morning"
		}
        else {
            
        }
	}
	checkIfNeedDate()
</script>