<script setup lang="ts">
import { Window } from '@tauri-apps/api/window'
import { ref, reactive, computed, onBeforeUnmount, onUnmounted, watch } from 'vue'
import { ElMessage, type FormItemProps, type FormProps } from 'element-plus'
import { convertFileSize } from '@/utils/fileUtil'

// 接收关闭状态
import { onMounted } from 'vue'
import type { ImageShowInfo } from '@/models/ImageShowInfo'
import { getImageInfo } from '@/services/imageService'
import type { ImageInfo } from '@/types/image.type'
import ImagePreview from '@/views/ImagePreview.vue'
import dayjs from 'dayjs'
import { ExpandError } from '@/errors'
import { convertFileSrc } from '@tauri-apps/api/core'
import StringUtils from '@/utils/stringUtils'
import ImageCacheManager from '@/utils/imageCacheManager'

// 接收关闭事件
const props = defineProps({
  imgInfo: Object as () => ImageShowInfo,
  // 关闭事件
  closePreview: Function,
  // 预览的图像列表
  images: Array as () => ImageShowInfo[],
  // 当前要预览的图片
  showImageIndex: Number
})
// 正在展示的图像【初始信息】
const previewImageInitInfo =ref<ImageShowInfo | undefined> (props.imgInfo)
// 正在预览的图像【详细信息】
const previewImage = ref<ImageShowInfo | undefined>(props.imgInfo)
// 是否展示详细信息
const isShowInfo = ref(true)
// 图像具体信息
const imageInfo = ref<ImageInfo | undefined>(undefined)
// 图像缓存管理
let instance: ImageCacheManager = ImageCacheManager.getInstance()

// 当前展示的图像 index
let showImageIndex = props.showImageIndex || 0
// 当前预览的图像列表
let showImagesList = props.images || []
// 下一张和上一张按下的次数
let nextImageCount = 0
let previousImageCount = 0

watch(
  previewImageInitInfo,
  (newValue, oldValue) => {
    if (newValue) {
      let str = newValue?.sourceFilePath
      if (StringUtils.isBlank(str)) {
        throw ExpandError.PathIsNullOrBlankError
      }

      newValue!.sourceFileShowPath = convertFileSrc(str!)
      previewImage.value = newValue
      getImageInfo(newValue!.sourceFilePath).then((res) => {
        imageInfo.value = JSON.parse(res)
      })
    }
  }
)

// 详细信息字段(请求数据库获取)
if (previewImageInitInfo) {
  getImageInfo(previewImageInitInfo.value!.sourceFilePath).then((res) => {
    imageInfo.value = JSON.parse(res)
  })
}

/**
 * 按键切换
 * @param event
 */
function handleKeydown(event: KeyboardEvent) {
  let idx = 0
  switch (event.key) {
    case 'ArrowUp':
      break
    case 'ArrowDown':
      break
    case 'ArrowLeft':
      idx = showImageIndex - 1
      if (idx < 0) {
        ElMessage.warning(ExpandError.IsFirstOneError.message)
        return
      }
      previewImageInitInfo.value = showImagesList[idx]
      showImageIndex = idx

      break
    case 'ArrowRight':
      idx = showImageIndex + 1
      if (idx >= showImagesList.length) {
        ElMessage.warning(ExpandError.IsLastOneError.message)
        return
      }
      previewImageInitInfo.value = showImagesList[idx]
      showImageIndex = idx
      break
  }
  imageCache(idx)

}

// 关闭预览
const closePreview = () => {
  // 显示滚动条
  document.body.style.overflow = 'auto'
  // 关闭预览
  props.closePreview!()
}
onMounted(() => {
  // 隐藏滚动条
  document.body.style.overflow = 'hidden'
  // 注册快捷键
  window.addEventListener('keydown', handleKeydown)
})
onUnmounted(() => {
  // 注销快捷键
  window.removeEventListener('keydown', handleKeydown)

  // 清空缓存
  instance.clearCache()
})

onMounted(() => {
  imageCache(0)
})

/**
 * 缓存图片【缓存当前展示图片前后的 指定 张图片】
 * @param imgIndex 当前展示图像的 index
 * @param imgNum 要缓存的图像张数,默认缓存前后10张
 */
function imageCache(imgIndex: number, imgNum: number = 10) {
  console.log("开始图像缓存");
  // 获取要缓存的图片范围
  const startIdx = Math.max(0, imgIndex - imgNum)
  const endIdx = Math.min(showImagesList.length, imgIndex + imgNum)

  // 缓存图片
  for (let i = startIdx; i < endIdx; i++) {
    const img = showImagesList[i]
    if (img && img.sourceFileShowPath) {
      instance.preloadImage(img.sourceFileShowPath)
    }
  }
}
// 全屏
async function setFullScreen() {
  let current = Window.getCurrent()
  await current.setFullscreen(true)
}
</script>

<template>
  <div
    @wheel.prevent
    @touchmove.prevent
    @keydown.prevent
    @blur.prevent
    @abort.prevent
    @scroll.prevent
    @drag.prevent
    @focus.prevent
    @mousedown.prevent
    class="fixed inset-0 z-50 bg-black bg-opacity-90"
  >
    <!--    图像预览-->
    <ImagePreview class="w-full h-full" :imgInfo="previewImage" />
    <!--    信息展示-->
    <div
      v-if="isShowInfo && imageInfo !== undefined && imageInfo !== null"
      class="text-white pointer-events-none font-light absolute top-0 left-0 z-51 p-4 pt-8"
    >
      <div v-if="imageInfo!.imgName">
        <span>文件名:</span>
        {{ imageInfo!.imgName }}
      </div>
      <div v-if="imageInfo!.fileSize">
        <span>图片大小:</span>
        {{ convertFileSize(imageInfo!.fileSize, 'B').size }}
        {{ convertFileSize(imageInfo!.fileSize, 'B').unit }}
      </div>
      <div v-if="imageInfo!.dateTimeOriginal">
        <span>修改日期:</span>
        {{ dayjs(imageInfo!.dateTimeOriginal).format('YYYY/MM/DD HH:mm:ss') }}
      </div>
      <div v-if="imageInfo!.height && imageInfo!.width">
        <span>图片信息:</span>
        {{ imageInfo!.height }}×{{ imageInfo!.width }} ({{ imageInfo!.format }})
      </div>
      <br />
      <div v-if="imageInfo!.make">
        <span>相机制造商:</span>
        {{ imageInfo!.make }}
      </div>
      <div v-if="imageInfo!.model">
        <span>相机型号:</span>
        {{ imageInfo!.model }}
      </div>
      <div v-if="imageInfo!.software">
        <span>软件:</span>
        {{ imageInfo!.software }}
      </div>
      <div v-if="imageInfo!.dateTimeOriginal">
        <span>拍摄日期:</span>
        {{ dayjs(imageInfo!.dateTimeOriginal).format('YYYY/MM/DD HH:mm:ss') }}
      </div>
      <div v-if="imageInfo!.flash">
        <span>闪光灯:</span>
        {{ imageInfo!.flash }}
      </div>
      <div v-if="imageInfo!.focalLength">
        <span>焦距:</span>
        {{ imageInfo!.focalLength }}mm (焦距 (35mm): {{ imageInfo!.focalLength }}mm)
      </div>
      <div v-if="imageInfo!.exposureTime">
        <span>快门速度:</span>
        {{ imageInfo!.exposureTime }}s (1/13)
      </div>
      <div v-if="imageInfo!.fNumber">
        <span>光圈数:</span>
        f/{{ imageInfo!.fNumber }}
      </div>
      <div v-if="imageInfo!.iso">
        <span>ISO 感光度:</span>
        {{ imageInfo!.iso }}
      </div>
      <div v-if="imageInfo!.exposureProgram">
        <span>曝光程序:</span>
        {{ imageInfo!.exposureProgram }}
      </div>
      <div v-if="imageInfo!.meteringMode">
        <span>测光模式:</span>
        {{ imageInfo!.meteringMode }}
      </div>
    </div>

    <!-- 关闭按钮 -->
    <button
      @click="closePreview"
      class="absolute top-5 right-5 bg-white text-black rounded-full p-2 pl-3.5 pr-3.5 hover:bg-gray-300"
    >
      ✖
    </button>
  </div>
</template>

<style scoped lang="scss">
.img-transition {
  transition: all 0.2s;
}
</style>
