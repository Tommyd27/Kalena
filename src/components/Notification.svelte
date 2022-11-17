
<script>
	import { fade } from 'svelte/transition';
	/*
	0 = error
	*/
	export let notificationType = 0
	export let notificationText = "ELLO"
	export let visible = false;
</script>

{#if visible}
	<p out:fade>
		{notificationText}
	</p>
{/if}


