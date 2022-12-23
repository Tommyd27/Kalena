<script>
	import { invoke } from "@tauri-apps/api/tauri"
    async function checkIfNeedDate()
	{
		if (await invoke('need_date'))
		{
			window.location.href = "/good-morning"
		}
        else {
            
        }
	}
	checkIfNeedDate()
</script>