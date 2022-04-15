<template>
  <div class="blog">
    <img alt="Vue logo" src="../assets/logo.svg" width="200">
    <BlogComponent msg="This is a blog"/>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import BlogComponent from '@/components/BlogComponent.vue'; // @ is an alias to /src

export default defineComponent({
  name: 'BlogView',
  components: {
    BlogComponent,
  },
});
</script>
