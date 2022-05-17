<script lang="ts">
  import { routes } from "./routes";
  import { Router } from "svelte-router-spa";
</script>

<main class="center">
  <Router {routes} options={{ prefix: "app" }} />
</main>

<style>
</style>
