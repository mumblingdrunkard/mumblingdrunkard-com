<script lang="ts">
</script>

<main>
  <h2>A small component</h2>
  <p>
    This is an eagerly loaded component.
    It is included in the main bundle and is always loaded.
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
