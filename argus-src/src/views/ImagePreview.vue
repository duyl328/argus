<script setup lang="ts">
import { Window } from '@tauri-apps/api/window'
import { ref, computed, onBeforeUnmount } from 'vue'

// 接收关闭状态
import { onMounted } from 'vue'
import type { ImageShowInfo } from '@/models/ImageShowInfo'
import { getImageInfo } from '@/services/imageService'
import type { ImageInfo } from '@/types/image.type'
// 接收关闭事件
const props = defineProps({
  imgInfo: Object as () => ImageShowInfo,
  closePreview: Function
})
// 正在预览的图像
const previewImage = ref<ImageShowInfo | undefined>(props.imgInfo)
// 是否展示详细信息
const isShowInfo = ref(false)

// 图像具体信息
const imageInfo = ref<ImageInfo | undefined>(undefined)

// 详细信息字段(请求数据库获取)
if (props.imgInfo) {
  getImageInfo(props.imgInfo.sourceFilePath).then((res) => {
    imageInfo.value = JSON.parse(res)
    windowSizeChange()
    console.log(imageStyle.value)
    console.log(imageInfo.value?.width, imageInfo.value?.height)
  })
}

// 关闭预览
const closePreview = () => {
  // 显示滚动条
  document.body.style.overflow = 'auto'
  // 关闭预览
  props.closePreview!()
}

// region 图像滚轮放大和拖动
// 图像样式
const imageStyle = ref('')
const imageCursorStyle = ref('')

// 图像宽高
let imgWidth = 0
let imgHeight = 0
// 最大缩放比例
let maxScale = 3
// 最小缩放比例
let minScale = 0.5
// 比例间隔
let scaleInterval = 0.2

// 拖动起始位置
let startX = 0
let startY = 0

// 是否正在拖动
let isDragging = false

// 是否被拖拽过
let isDragged = false

// 图像的当前偏移位置
let offsetX = 0
let offsetY = 0

// 图像记录的偏移位置
let lastOffsetX = 0
let lastOffsetY = 0

// 图像是否在视口
let isInViewport = ref(true)
// 图像最少在画面中出现的大小
let minSize = 100

// 获取容器和图像的引用
const imageWrapper = ref(undefined)
const imageContainer = ref(undefined)

// 图像样式
const computeImageStyle = function () {
  let size = 'width:' + imgWidth + 'px;height:' + imgHeight + 'px;'
  // 根据图像大小，计算偏移量
  if (!isDragged) {
    if (imgWidth > window.innerWidth) {
      offsetX = Math.min(Math.max(offsetX, window.innerWidth - imgWidth), 0)
    } else {
      offsetX = (window.innerWidth - imgWidth) / 2
    }
    if (imgHeight > window.innerHeight) {
      offsetY = Math.min(Math.max(offsetY, window.innerHeight - imgHeight), 0)
    } else {
      offsetY = (window.innerHeight - imgHeight) / 2
    }

    lastOffsetX = offsetX
    lastOffsetY = offsetY
  }
  imageStyle.value = 'left:' + offsetX + 'px;' + 'top:' + offsetY + 'px;' + size

  // 计算图像是否在视口内
  isInViewport.value =
    window.innerWidth - offsetX > minSize &&
    window.innerHeight - offsetY > minSize &&
    imgWidth + offsetX > minSize &&
    imgHeight + offsetY > minSize
}

// 重置图片位置
const resetImagePosition = function () {
  offsetX = 0
  offsetY = 0
  isDragged = false
  windowSizeChange()
}

// 计算鼠标样式
const computeCursorStyle = function () {
  imageCursorStyle.value = 'cursor:' + (isDragging ? 'grabbing' : 'grab') + ';'
}

computeCursorStyle()

// 鼠标滚轮事件，缩放图像
function onWheel(event: WheelEvent) {
  event.preventDefault() // 阻止默认的滚动行为
  let scale = 1
  let d = 0
  if (event.deltaY > 0) {
    scale = Math.max(minScale, scale - scaleInterval) // 向下滚动，缩小，保证不小于 minScale
    d = -scaleInterval
  } else {
    scale = Math.min(maxScale, scale + scaleInterval) // 向上滚动，放大，保证不大于 maxScale
    d = scaleInterval
  }

  // 按照图片中心放大缩小
  // offsetX += (imgWidth - imgWidth * scale) / 2
  // offsetY += (imgHeight - imgHeight * scale) / 2

  // 按照鼠标位置放大缩小
  // todo: 2025/2/16 20:44 按照屏幕中心的位置进行放大

  imgWidth *= scale
  imgHeight *= scale
  // lastOffsetX = offsetX
  // lastOffsetY = offsetY

  computeImageStyle()
}

// 鼠标按下开始拖动
function onMouseDown(event: MouseEvent) {
  if (event.button === 0) {
    isDragging = true
    startX = event.clientX
    startY = event.clientY
    computeCursorStyle()
  }
  // 鼠标中间
  if (event.button === 1) {
    // 记录位置
    lastOffsetX = offsetX = 0
    lastOffsetY = offsetY = 0
  }
}

// 鼠标松开停止拖动
function onMouseUp() {
  isDragging = false
  // 鼠标抬起记录位置
  lastOffsetX = offsetX
  lastOffsetY = offsetY
  computeCursorStyle()
}

// 鼠标移动时拖动图像
function onMouseMove(event: MouseEvent) {
  isDragged = true
  if (isDragging) {
    offsetX = lastOffsetX + event.clientX - startX
    offsetY = lastOffsetY + event.clientY - startY
    computeImageStyle()
  }
}

// 鼠标移出元素，取消拖动事件
function onMouseLeave() {
  if (isDragging) {
    onMouseUp()
  }
}

// endregion

// 监听窗口变动事件（调整图像展示大小）
function windowSizeChange() {
  // 计算图像展示的的大小
  let width = window.innerWidth
  let height = window.innerHeight
  // 图像的宽高
  let imageWidth = imageInfo.value?.width || 0
  let imageHeight = imageInfo.value?.height || 0
  // 图像的比例
  let imageScale = imageWidth / imageHeight
  // 屏幕的比例
  let screenScale = width / height
  // 图像的宽度
  let newWidth = 0
  let newHeight = 0
  // 当图像清晰度更大时，则压缩整体比例；反之，按照图像宽度进行内容展示
  if (imageWidth > width || imageHeight > height) {
    if (imageScale > screenScale) {
      newWidth = width
      newHeight = width / imageScale
    } else {
      newHeight = height
      newWidth = height * imageScale
    }
  } else {
    newWidth = imageWidth
    newHeight = imageHeight
  }
  imgWidth = newWidth
  imgHeight = newHeight
  computeImageStyle()
}

// 全屏
async function setFullScreen() {
  let current = Window.getCurrent()
  await current.setFullscreen(true)
}

// setFullScreen()

onMounted(() => {
  // 窗口变动事件
  window.addEventListener('resize', windowSizeChange)
  // 隐藏滚动条
  document.body.style.overflow = 'hidden'
})

onBeforeUnmount(() => {
  // 窗口变动事件
  window.removeEventListener('resize', windowSizeChange)
})
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex justify-center items-center bg-black bg-opacity-70"
    @wheel.prevent
    @touchmove.prevent
    @keydown.prevent
    @click.prevent
    @blur.prevent
    @abort.prevent
    @scroll.prevent
    @drag.prevent
    @focus.prevent
    @mousedown.prevent
  >
    <div v-if="previewImage !== undefined" class="relative w-full h-full flex flex-row" @click.stop>
      <!-- 预览图片 -->
      <div
        @wheel="onWheel($event)"
        @mousedown="onMouseDown($event)"
        @mouseup="onMouseUp"
        @mousemove="onMouseMove($event)"
        @mouseleave="onMouseLeave"
        ref="imageContainer"
        :style="imageCursorStyle"
        class="flex-1 relative bg-red-200 p-4"
      >
        <img
          ref="imageWrapper"
          class="absolute top-0 left-0 img-transition max-w-fit"
          :src="previewImage.sourceFileShowPath"
          alt="Image"
          :style="imageStyle"
        />
      </div>
      <!--
      上一个
      下一个
      放大
      缩小
      旋转（左右）
      镜像（待定）
      -->

      <!-- 操作菜单 -->
      <div class="w-80 bg-green-200 p-4" :class="isShowInfo ? 'block' : 'hidden'"></div>
      <!--  重置图片位置 -->
      <el-button
        v-if="!isInViewport"
        @click="resetImagePosition"
        class="absolute bottom-10 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
        >重置图片
      </el-button>

      <!-- 关闭按钮 -->
      <button
        @click="closePreview"
        class="absolute top-12 right-12 bg-white text-black rounded-full p-2 pl-3.5 pr-3.5 hover:bg-gray-300"
      >
        ✖
      </button>
    </div>
    <!--    图像预览失败-->
    <div v-else>图像预览失败</div>
  </div>
</template>

<style scoped lang="scss">
.img-transition {
  //transition: all 0.2s
}
</style>
