<script setup lang="ts">
import { ref, reactive, onMounted, watch } from 'vue'
import { getDirAllSubfoldersFirstImg } from '@/services/folderService'
import type { FolderImage } from '@/types/rusts/FolderImage'

const images = ref<FolderImage[]>([])

const columns = ref<number>(3) // 默认列数
const columnImages = reactive<FolderImage[][]>([]) // 每列的图片内容

// 动态分配图片到列
const distributeImages = () => {
  // 初始化列数组
  const cols = Array.from({ length: columns.value }, () => [] as FolderImage[])
  images.value.forEach((image, index) => {
    cols[index % columns.value].push(image) // 分配到列中
  })
  columnImages.length = 0
  columnImages.push(...cols)
}

// 每列合适的大小
const updateColumns = () => {
  const width = window.innerWidth
  if (width >= 2400)
    columns.value = 11 // 2xl
  else if (width >= 1536)
    columns.value = 9 // 2xl
  else if (width >= 1280)
    columns.value = 7 // xl
  else if (width >= 1024)
    columns.value = 6 // lg
  else if (width >= 768)
    columns.value = 5 // md
  else if (width >= 640)
    columns.value = 4 // sm
  else columns.value = 3 // 默认

  distributeImages()
}

onMounted(() => {
  // let dirAllSubfoldersFirstImg = getDirAllSubfoldersFirstImg('E:\\整合\\niannian 125套\\年年（vip套图）')
  let dirAllSubfoldersFirstImg = getDirAllSubfoldersFirstImg('D:\\argus\\img')
  dirAllSubfoldersFirstImg.then((res) => {
    console.log(res)
    images.value = [...res]
    updateColumns() // 初始化时计算列数
  })
  window.addEventListener('resize', updateColumns) // 监听窗口变化
})

watch(columns, distributeImages) // 列数变化时重新分配图片
</script>

<template>
  <div>
    <!-- 瀑布流主容器 -->
    <div class="grid p-5" :style="{ gridTemplateColumns: `repeat(${columns}, 1fr)` }">
      <!-- 每列的内容 -->
      <div
        v-for="(col, index) in columnImages"
        :key="index"
        class="flex flex-col gap-4 m-2 rounded-lg shadow"
      >
        <div v-for="(image, idx) in col" :key="idx">
          <img
            :src="'data:image/jpeg;base64,' + image.image_path_as_base64"
            alt="Image"
            class="w-full rounded-lg object-cover shadow"
          />
          <span>{{ image.folder_path }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<!-- ======================== TODO【Masonry 使用示例 】 ===================================== -->

<!--<script setup>-->
<!--// 图片地址-->
<!--import { onMounted, ref } from 'vue'-->
<!--import { getDirAllSubfoldersFirstImg } from '@/services/folderService'-->
<!--import Masonry from 'masonry-layout'-->

<!--const masonryContainer = ref(null)-->

<!--let img = {-->
<!--  name: '',-->
<!--  path: 'D:/Camera/Camera/Camera',-->
<!--  img: '/src/assets/images/img_example_3.png'-->
<!--}-->
<!--const imgs = ref([-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-2.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-2.png',-->
<!--  '/src/assets/images/img-2.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-2.png',-->
<!--  '/src/assets/images/img-2.png',-->
<!--  '/src/assets/images/img-2.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-1.png',-->
<!--  '/src/assets/images/img-1.png'-->
<!--])-->

<!--function waitForImagesToLoad() {-->
<!--  const images = masonryContainer.value.querySelectorAll('img')-->
<!--  let loadedCount = 0-->

<!--  images.forEach((img) => {-->
<!--    // 使用 Image 对象监听图片加载-->
<!--    const imageLoader = new Image()-->
<!--    imageLoader.src = img.src-->
<!--    imageLoader.onload = imageLoader.onerror = () => {-->
<!--      loadedCount++-->
<!--      if (loadedCount === images.length) {-->
<!--        // 全部图片加载完成后初始化 Masonry-->
<!--        masonryInstance()-->
<!--      }-->
<!--    }-->
<!--  })-->
<!--}-->

<!--function masonryInstance() {-->
<!--  new Masonry(masonryContainer.value, {-->
<!--    itemSelector: '.masonry-item',-->
<!--    columnWidth: 120,-->
<!--    // percentPosition: true,-->
<!--    gutter: 16, // Gap between items-->
<!--    fitWidth: true-->
<!--  })-->
<!--}-->

<!--onMounted(() => {-->
<!--  waitForImagesToLoad()-->
<!--})-->
<!--</script>-->

<!--<template>-->
<!--  <div ref="masonryContainer">-->
<!--    <div v-for="item in imgs" class="masonry-item">-->
<!--      <img class="" :src="item" alt="Example" />-->
<!--      <span>{{ item }}</span>-->
<!--    </div>-->
<!--  </div>-->
<!--</template>-->
