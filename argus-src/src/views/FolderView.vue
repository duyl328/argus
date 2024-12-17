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

// 屏幕宽度判断
const colJudgement = [
  { width: 2400, col: 13 },
  { width: 1536, col: 9 },
  { width: 1280, col: 7 },
  { width: 1024, col: 6 },
  { width: 768, col: 5 },
  { width: 640, col: 4 },
  { width: -1, col: 3 }
]

// 计算图片最宽的宽度
function updateColumns() {
  const width = window.innerWidth
  for (let i = 0; i < colJudgement.length; i++) {
    let item = colJudgement[i]
    if (width >= item.width) {
      console.log(width, item.width);
      distributeImages()
      columns.value = item.col
      return
    }
  }
}

// 寻找最宽的图片大小
function getMaxImgWidth() {
  let ans = 0
  colJudgement.forEach((image, index) => {
    if (image.width <= 0) return ans
    let result = image.width / image.col
    console.log(result)
    if (result > ans) {
      ans = result
    }
  })
  return ans
}

onMounted(() => {
  let maxImgWidth = getMaxImgWidth()
  console.log(maxImgWidth)
  // let dirAllSubfoldersFirstImg = getDirAllSubfoldersFirstImg('E:\\整合\\niannian 125套\\年年（vip套图）')
  let dirAllSubfoldersFirstImg = getDirAllSubfoldersFirstImg(
    'D:\\argus\\img',
    Math.round(maxImgWidth),
    2400
  )
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
