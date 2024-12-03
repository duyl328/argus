<script setup lang="ts">

// 切换主题
function getSwitch () {
  let html = document.querySelector('html')
  let currentTheme = html.getAttribute('data-theme');

  if (currentTheme === "light") {
    html.setAttribute('data-theme', 'dark' );
  } else {
    html.setAttribute('data-theme', 'light');
  }
}
</script>

<template>
  <main>
    <h1 class="text-7xl">
      主页
    </h1>

<!--    <ElButton @click="getSwitch">切换主题</ElButton>-->
    <el-button />
  </main>
</template>
