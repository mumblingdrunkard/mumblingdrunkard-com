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
  main {
    text-align: center;
    padding: 1em;
    max-width: 640px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
