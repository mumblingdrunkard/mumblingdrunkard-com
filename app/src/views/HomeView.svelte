<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import { onMount } from "svelte";

  let Test = null;
  onMount(async () => {
    if (Test == null)
      Test = (await import("../components/Test.svelte")).default;
  });
</script>

<main>
  <h1>Home</h1>

  <p><Navigate to="blog">Link to blog!</Navigate></p>

  <svelte:component this={Test} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
