<script lang="ts">
  import { Router } from "svelte-router-spa";
  import { routes } from "./routes";
</script>

<main class="center">
  <p>
    See the source code for this on
    <a href="https://github.com/mumblingdrunkard/mumblingdrunkard-com">
      GitHub
    </a>
  </p>

  <Router {routes} options={{ prefix: "app" }} />
</main>

<style>
</style>
