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
    class="grid gap-4 grid-cols-1 m-2 sm:grid-cols-3  md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8"
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
