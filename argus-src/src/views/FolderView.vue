<script setup lang="ts">
import { ref, reactive, onMounted, watch, onUnmounted } from 'vue'
import { getAllImgs, getDirAllSubfoldersFirstImg } from '@/services/folderService'
import type { FolderImage } from '@/types/rusts/FolderImage'
import EmitOrder from '@/constants/emitOrder'
import { convertFileSrc } from '@tauri-apps/api/core'
import { Picture as IconPicture } from '@element-plus/icons-vue'
import { getImageThumbnail } from '@/services/imageService'
import LazyImage from '@/components/LazyImage.vue'
import { ImageShowInfo } from '@/models/ImageShowInfo'
import ImagePreview from '@/views/ImagePreview.vue'

const images = ref<ImageShowInfo[]>([])
// 图像预览数组
const preImages = ref<string[]>([])
// 默认列数
const columns = ref<number>(3)
// 是否进行预览
const isPreview = ref<boolean>(false)
// 进行预览的图片
const previewImage = ref<ImageShowInfo | undefined>(undefined)

/**
 * 预览关闭
 */
const closePreview = () => {
  isPreview.value = false
}

/**
 * 预览打开
 */
const openPreview = (info: ImageShowInfo) => {
  previewImage.value = info
  isPreview.value = true
}

// 屏幕宽度判断
const colJudgement = [
  { width: 2400, col: 9 },
  { width: 1900, col: 8 },
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
  let dirAllSubfoldersFirstImg = getAllImgs(
    // 'D:\\argus\\img\\jpg\\局部\\新建文件夹'
    'D:\\argus\\img'
    // 'D:\\argus\\argus-src\\src-tauri\\target\\debug\\局部'
    // 'E:\\整合',
  )

  dirAllSubfoldersFirstImg.then((res) => {
    res.forEach((item) => {
      let imageShowInfo = new ImageShowInfo(item.source_file_path, item.file_path)
      images.value.push(imageShowInfo)
    })
  })

  window.addEventListener('resize', updateColumns) // 监听窗口变化
})

onUnmounted(() => {
  window.removeEventListener('resize', updateColumns) // 清除监听窗口变化
})

/**
 * 展示的图像内容获取
 * @param info
 */
const setItemRef = (info: ImageShowInfo) => (el: Element) => {
  if (info.isWhether) return undefined
  info.isWhether = true
  if (el) {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          getImageThumbnail(info.sourceFilePath)
            .then((res) => {
              info.isLoading = false
              info.compressedPath = convertFileSrc(res)
              info.sourceFileShowPath = convertFileSrc(info.sourceFilePath)
              preImages.value.push(info.sourceFileShowPath)
            })
            .catch((err) => {
              info.isLoading = false
              info.isError = true
              info.errorMsg = err.toString()
              console.error(err)
            })
            .finally(() => {
              observer.unobserve(el)
            })
        }
      },
      {
        threshold: [0] // 当元素至少有 0 进入视口时触发回调
      }
    )
    observer.observe(el)
  }
  return undefined
}

updateColumns()
</script>

<template>
  <div>
    <!-- 瀑布流主容器 -->
    <div :style="{ gridTemplateColumns: `repeat(${columns}, 1fr)` }" class="grid">
      <!-- 每列的内容 -->
      <div
        v-for="(col, index) in images"
        :key="index"
        :ref="setItemRef(col)"
        class="flex flex-col gap-4 m-2 rounded-lg shadow overflow-hidden bg-image-show-text-bg"
      >
        <!--        Loading-->
        <!-- 使用 aspect-ratio 保证容器是方形的 -->
        <div v-if="col.isLoading" class="relative w-full" style="padding-top: 100%">
          <!-- 通过 padding-top 设置容器比例 -->
          <img
            src="@/assets/images/img_example.png"
            alt="Image"
            class="absolute top-0 left-0 w-full h-full object-cover rounded-t-lg img-load"
          />
        </div>
        <!--        失败-->
        <div v-else-if="col.isError" class="relative w-full" style="padding-top: 100%">
          <img
            src="@/assets/images/img_example_not_exist.png"
            alt="Image"
            class="absolute top-0 left-0 w-full h-full object-cover rounded-t-lg img-load"
          />
        </div>
        <!--        成功-->
        <div v-else @click="openPreview(col)" class="relative w-full" style="padding-top: 100%">
          <img
            :src="col.compressedPath"
            alt="Image"
            class="absolute top-0 left-0 w-full h-full object-cover rounded-t-lg"
          />
        </div>

        <!--        暂时不展示 tooltip -->
        <!--        <el-tooltip :content="col.filePath" placement="bottom" >-->
        <span
          class="ellipsis mb-3 mr-3 ml-1 whitespace-nowrap overflow-hidden text-ellipsis text-left"
        >
          {{ col.folderPath }}
        </span>
        <!--        </el-tooltip>-->
      </div>
    </div>
  </div>

  <ImagePreview v-if="isPreview" :closePreview="closePreview" :imgInfo="previewImage" />
</template>

<style scoped lang="scss">
// 图片加载失败
.img-load {
  width: 50%;
  height: 50%;
  transform: translate(50%, 50%);
  @apply rounded-lg;
}

.ellipsis {
  direction: rtl; /* 设置文本为右到左 */
}
</style>

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
