<template>
  <div class="blog" v-html="content"></div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { marked } from 'marked';
import { useStore } from 'vuex'
import { key } from '../store'

export default defineComponent({
  name: 'BlogView',
  components: {
  },
  data() {
    return {
      content: "Loading content...",
    }
  },
  async created() {
    const store = useStore(key)
    const path = "/doc/post.md"
    if (store.state.posts.has(path)) {
      const val = store.state.posts.get(path)
      if (typeof val === "string") {
        this.content = marked.parse(val)
      }
    } else {
      let response = await fetch("/doc/post.md")
      if (response.status == 200) {
        let text = await response.text()
        this.content = marked.parse(text)
        store.state.posts.set(path, text)
      }
    }
  },
  methods: {
  },
});
</script>

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
