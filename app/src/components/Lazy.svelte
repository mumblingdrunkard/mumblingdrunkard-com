<script lang="ts">
</script>

<main>
  <h2>A different component</h2>
  <p>
    This component is lazily loaded.
    It is not fetched until it is needed for rendering.
  </p>
</main>

<style>
  main {
    text-align: center;
    padding: .5em;
    max-width: 480px;
    margin: 0 auto;
  }

  main>p {
    text-align: justify;
  }

  h2 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 2em;
    font-weight: 100;
  }
</style>
