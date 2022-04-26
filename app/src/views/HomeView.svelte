<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import Eager from "../components/Eager.svelte"
</script>

<main class="center">
  <h1>Home</h1>

  <p><Navigate to="blog">Link to blog!</Navigate></p>

  <svelte:component this={Eager} />
</main>

<style>
</style>
