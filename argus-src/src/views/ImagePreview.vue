<script setup lang="ts">
import { Window } from '@tauri-apps/api/window'
import { ref, computed, onBeforeUnmount } from 'vue'

// 接收关闭状态
import { onMounted } from 'vue'
import type { ImageShowInfo } from '@/models/ImageShowInfo'
// 接收关闭事件
const props = defineProps({
  imgInfo: Object as () => ImageShowInfo,
  closePreview: Function
})
// 正在预览的图像
const previewImage = ref<ImageShowInfo | undefined>(props.imgInfo)
// 是否展示详细信息
const isShowInfo = ref(false)

// todo: 2025/2/15 18:41 请求图像信息（图像基础信息，包括 名称、宽高 等）

// 详细信息字段(请求数据库获取)

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
// 当前缩放比例
let scale = 1
// 最大缩放比例
let maxScale = 3
// 最小缩放比例
let minScale = 0.5

// 拖动起始位置
let startX = 0
let startY = 0

// 是否正在拖动
let isDragging = false

// 图像的当前偏移位置
let offsetX = 0
let offsetY = 0

// 图像记录的偏移位置
let lastOffsetX = 0
let lastOffsetY = 0

// 获取容器和图像的引用
const imageWrapper = ref(undefined)

// 图像样式
const computeImageStyle = function () {
  // {
  //   transform: `translate(${offsetX}px, ${offsetY}px)`,
  //
  // }
  imageStyle.value =
    'translate:' +
    offsetX +
    'px ' +
    offsetY +
    'px;' +
    'cursor: ' +
    (isDragging ? 'grabbing' : 'grab') +
    ';'
}
computeImageStyle()

// 鼠标滚轮事件，缩放图像
function onWheel(event: WheelEvent) {
  event.preventDefault()

  // 计算放大缩小的比例
  let newScale = scale + event.deltaY * -0.01
  newScale = Math.min(Math.max(newScale, minScale), maxScale)

  // 根据鼠标位置调整缩放中心
  const scaleDiff = newScale / scale
  offsetX = offsetX - (event.offsetX - offsetX) * scaleDiff
  offsetY = offsetY - (event.offsetY - offsetY) * scaleDiff

  scale = newScale
}

// 鼠标按下开始拖动
function onMouseDown(event: MouseEvent) {
  isDragging = true
  startX = event.clientX
  startY = event.clientY

  // 鼠标中间
  if (event.button === 1) {
    // 记录位置
    lastOffsetX = offsetX = 0
    lastOffsetY = offsetY = 0
  }
  computeImageStyle()
}

// 鼠标松开停止拖动
function onMouseUp() {
  isDragging = false
  // 鼠标抬起记录位置
  lastOffsetX = offsetX
  lastOffsetY = offsetY
  computeImageStyle()
}

// 是否还在视口
let observer: IntersectionObserver | null = null
const handleIntersection = (entries: IntersectionObserverEntry[]) => {
  entries.forEach((entry) => {
    // todo: 2025/2/15 22:05 离开视口时展示重置按钮
    if (entry.isIntersecting) {
      console.log('元素进入可视窗口')
    } else {
      console.log('元素离开可视窗口')
    }
  })
}

// 鼠标移动时拖动图像
// todo: 2025/2/15 21:51 鼠标移出元素，取消拖动事件
function onMouseMove(event: MouseEvent) {
  if (isDragging) {
    offsetX = lastOffsetX + event.clientX - startX
    offsetY = lastOffsetY + event.clientY - startY
    computeImageStyle()
  }
}

// endregion
// 全屏
async function setFullScreen() {
  let current = Window.getCurrent()
  await current.setFullscreen(true)
}

// setFullScreen()

onMounted(() => {
  // 隐藏滚动条
  document.body.style.overflow = 'hidden'

  // IntersectionObserver配置 【监听是否还在页面展示】
  const options: IntersectionObserverInit = {
    root: null, // 观察视口
    rootMargin: '0px',
    threshold: 0.2 // 50% 进入视口时触发
  }

  observer = new IntersectionObserver(handleIntersection, options)

  if (imageWrapper.value) {
    observer.observe(imageWrapper.value)
  }
})

onBeforeUnmount(() => {
  // 取消监视
  if (observer && imageWrapper.value) {
    observer.disconnect()
  }
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
        class="flex-1 relative bg-blue-200 p-4"
      >
        <img
          ref="imageWrapper"
          class="absolute top-0 left-0 h-full w-full object-contain rounded-lg p-5"
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

<style scoped></style>
