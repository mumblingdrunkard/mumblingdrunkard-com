<template>
  <div class="blog">
    <h1>Blog</h1>
    <p>Lorem ipsum dolor sit amet</p>
    <p>{{ msg }}</p>
    <router-link to="/blog/post.md">A post</router-link>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'BlogComponent',
  props: {
    msg: String,
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
.content {
  width: 450px;
  margin: auto;
  text-align: left;
}
</style>
