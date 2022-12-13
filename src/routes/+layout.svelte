<script>
  import "../app.css";
  import KalenaIcon from "$lib/KalenaIcon.svelte";
  import NavBar from "$lib/NavBar.svelte";
</script>

<main class="container">
  <KalenaIcon></KalenaIcon>
  <NavBar></NavBar>
  <slot />
</main>
