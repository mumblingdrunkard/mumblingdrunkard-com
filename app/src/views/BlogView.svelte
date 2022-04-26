<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import Eager from "../components/Eager.svelte";
  import { onMount } from "svelte";

  let Lazy = null;
  onMount(async () => {
    Lazy = (await import("../components/Lazy.svelte")).default;
  });
</script>

<main class="center">
  <h1>The Blog!</h1>

  <Navigate to="/">Link to home!</Navigate>

  <svelte:component this={Eager} />
  <svelte:component this={Lazy} />
</main>

<style>
</style>
