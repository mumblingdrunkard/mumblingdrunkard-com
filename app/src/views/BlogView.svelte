<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import Post from '../components/Post.svelte'
</script>

<main class="center">
  <h1>The Blog!</h1>

  <Navigate to="/">Link to home!</Navigate>

  <svelte:component this={Post} />
</main>

<style>
</style>
