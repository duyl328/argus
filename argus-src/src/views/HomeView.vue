<script setup lang="ts">

// 切换主题
import { greet } from '@/services/base'
import { useRouter } from 'vue-router'


function getSwitch () {

  let stringPromise = greet('你好啊！！')
  stringPromise.then((value) => {
    console.log(value)
  })

  let html = document.querySelector('html')!
  let currentTheme = html.getAttribute('data-theme')

  if (currentTheme === 'light') {
    html.setAttribute('data-theme', 'dark')
  } else {
    html.setAttribute('data-theme', 'light')
  }
}

</script>

<template>
  <main>
    <h1 class="text-7xl">
      主页
    </h1>

    <ElButton @click="getSwitch">切换主题</ElButton>
  </main>
</template>
