<script lang="ts">
  import { onMount } from "svelte";

  let content = "Nothing here!";
  onMount(async () => {
    fetch("/api/test", { cache: "force-cache" }).then((response) => {
      if (response.status == 200) {
        response.json().then((json) => {
          content = json.content;
        });
      }
    });
  });
</script>

<main class="post">
  {@html content}
</main>

<style>
</style>
