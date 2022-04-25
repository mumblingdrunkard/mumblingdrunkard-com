<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import { onMount } from "svelte";

  let Test = null,
    Test2 = null;
  onMount(async () => {
    if (Test == null)
      Test = (await import("../components/Test.svelte")).default;
    if (Test2 == null)
      Test2 = (await import("../components/Test2.svelte")).default;
  });
</script>

<main>
  <h1>The Blog!</h1>

  <p><Navigate to="/">Link to home!</Navigate></p>

  <svelte:component this={Test} />
  <svelte:component this={Test2} />
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
