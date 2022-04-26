<script lang="ts">
  import LazyImported from './LazyImported.svelte';
</script>

<main>
  <h2>Another lazy component</h2>
  <p>
    This component is lazily loaded.
    It is not fetched until it is needed for rendering.
  </p>

  <LazyImported />
</main>

<style>
</style>
