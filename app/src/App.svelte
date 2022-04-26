<script lang="ts">
  import { Router } from "svelte-router-spa";
  import { routes } from "./routes";
</script>

<main>
  <Router {routes} options={{ prefix: "app" }} />

  <p>
    See the source code for this on
    <a href="https://github.com/mumblingdrunkard/mumblingdrunkard-com">
      GitHub
    </a>
  </p>
</main>

<style>
</style>
