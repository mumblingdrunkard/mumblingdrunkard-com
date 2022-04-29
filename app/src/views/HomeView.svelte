<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import Eager from "../components/Eager.svelte"
</script>

<main class="center">
  <h1>Home</h1>

  <Navigate to="blog">Link to blog!</Navigate>

  <svelte:component this={Eager} />
</main>

<style>
</style>
