<script lang="ts">
  import { Route, Navigate } from "svelte-router-spa";
  export let currentRoute;
  const params = {};
</script>

<main class="center">
  <nav>
    <Navigate to="/">Home</Navigate>
    <Navigate to="blog">Blog</Navigate>
    <Navigate to="projects">Projects</Navigate>
    <Navigate to="about">About</Navigate>
    <Navigate to="contact">Contact</Navigate>
  </nav>

  <Route {currentRoute} {params} />
</main>

<style>
</style>
