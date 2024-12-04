<script setup lang="ts">
    let str = "486486";

</script>

<template>
  <div class="about">
    <h1>This is an about page</h1>
    <h1>{{ str}}</h1>
  </div>
</template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
