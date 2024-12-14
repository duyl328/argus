<script setup lang="ts">
// 图片地址
import { ref } from 'vue'

let img = {
  name: '',
  path: 'D:/Camera/Camera/Camera',
  img: '/src/assets/images/img_example_3.png'
}

let imgPaths = ref([
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img,
  img
])
const width = window.innerWidth

console.log(width)
</script>

<template>
  <!--  顶部状态展示-->
  <h1>路径图片展示</h1>

  <!--  sm md lg xl 2xl-->

  <!--  图像展示-->
  <div
    class="grid gap-4 p-5 grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-7 2xl:grid-cols-9"
  >
    <img
      v-for="item in imgPaths"
      class="m-2 w-full h-auto object-cover bg-gray-100 rounded-lg shadow"
      :src="item.img"
      alt="Example"
    />
  </div>
</template>

<style scoped></style>
