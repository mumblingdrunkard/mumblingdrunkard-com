<script lang="ts">
  import { Navigate } from "svelte-router-spa";
  import Eager from "../components/Eager.svelte";
  import { onMount } from "svelte";

  let Lazy = null;
  onMount(async () => {
    Lazy = (await import("../components/Lazy.svelte")).default;
  });
</script>

<main>
  <h1>The Blog!</h1>

  <p><Navigate to="/">Link to home!</Navigate></p>

  <svelte:component this={Eager} />
  <svelte:component this={Lazy} />
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
