<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import { onMount } from "svelte";

  let Post = null;
  onMount(async () => {
    Post = (await import("../components/Post.svelte")).default;
  });
</script>

<main class="center">
  <h1>The Blog!</h1>

  <Navigate to="/">Link to home!</Navigate>

  <svelte:component this={Post} />
</main>

<style>
</style>
