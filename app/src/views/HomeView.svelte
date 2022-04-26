<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import Eager from "../components/Eager.svelte"
  import { onMount } from "svelte";

  let Lazy2 = null;
  onMount(async () => {
    Lazy2 = (await import("../components/Lazy2.svelte")).default;
  });
</script>

<main class="center">
  <h1>Home</h1>

  <Navigate to="blog">Link to blog!</Navigate>

  <svelte:component this={Eager} />
  <svelte:component this={Lazy2} />
</main>

<style>
</style>
