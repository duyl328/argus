<script setup lang="ts">
import { ref, reactive, onMounted, watch } from 'vue'
import { getDirAllSubfoldersFirstImg } from '@/services/folderService'
import type { FolderImage } from '@/types/rusts/FolderImage'
import { addListener } from '@/services/emits/base'
import EmitOrder from '@/constants/emitOrder'
import { convertFileSrc } from '@tauri-apps/api/core'
import { Picture as IconPicture } from '@element-plus/icons-vue'
import { getImageThumbnail } from '@/services/imageService'

const images = ref<string[]>([])
const columns = ref<number>(3) // 默认列数

// 屏幕宽度判断
const colJudgement = [
  { width: 2400, col: 11 },
  { width: 1536, col: 7 },
  { width: 1280, col: 5 },
  { width: 1024, col: 4 },
  { width: 768, col: 3 },
  { width: 640, col: 3 },
  { width: -1, col: 3 }
]

function updateColumns() {
  const width = window.innerWidth
  for (let i = 0; i < colJudgement.length; i++) {
    let item = colJudgement[i]
    if (width >= item.width) {
      columns.value = item.col
      return
    }
  }
}

onMounted(() => {
  let dirAllSubfoldersFirstImg = getDirAllSubfoldersFirstImg(
    'D:\\argus\\img\\jpg'
    // 'E:\\整合\\niannian 125套\\年年（vip套图）',
  )

  dirAllSubfoldersFirstImg.then((res) => {
    images.value = [...res]
  })
  window.addEventListener('resize', updateColumns) // 监听窗口变化
})
updateColumns()

async function getImg(imgPath: string) {
  try {
    let imageThumbnailPath = await getImageThumbnail(imgPath)
    return convertFileSrc(imageThumbnailPath)
  } catch (err) {
    return ''
  }
}
</script>

<template>
  <div>
    <!-- 瀑布流主容器 -->
    <div :style="{ gridTemplateColumns: `repeat(${columns}, 1fr)` }" class="grid">
      <!-- 每列的内容 -->
      <div
        v-for="(col, index) in images"
        :key="index"
        class="flex bg-blue-400 flex-col gap-4 m-2 rounded-lg shadow overflow-hidden"
      >
        <!-- 使用 aspect-ratio 保证容器是方形的 -->
        <div class="relative w-full bg-green-400" style="padding-top: 100%">



          <!-- 通过 padding-top 设置容器比例 -->

          <img
            v-argus-lazy="col"
            src="@/assets/images/img_example.png"
            alt="Image"
            class="absolute top-0 left-0 w-full h-full object-cover rounded-lg shadow"
          />
        </div>

        <span>{{ col }}</span>
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
