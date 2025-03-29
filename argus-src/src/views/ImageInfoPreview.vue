<script setup lang="ts">
import { Window } from '@tauri-apps/api/window'
import { ref, reactive, computed, onBeforeUnmount } from 'vue'
import type { FormItemProps, FormProps } from 'element-plus'
import { convertFileSize } from '@/utils/fileUtil'

// 接收关闭状态
import { onMounted } from 'vue'
import type { ImageShowInfo } from '@/models/ImageShowInfo'
import { getImageInfo } from '@/services/imageService'
import type { ImageInfo } from '@/types/image.type'
import ImagePreview from '@/views/ImagePreview.vue'
import dayjs from 'dayjs'

// 接收关闭事件
const props = defineProps({
  imgInfo: Object as () => ImageShowInfo,
  closePreview: Function
})
// 正在预览的图像
const previewImage = ref<ImageShowInfo | undefined>(props.imgInfo)
// 是否展示详细信息
const isShowInfo = ref(true)

// 图像具体信息
const imageInfo = ref<ImageInfo | undefined>(undefined)

// 详细信息字段(请求数据库获取)
if (props.imgInfo) {
  getImageInfo(props.imgInfo.sourceFilePath).then((res) => {
    imageInfo.value = JSON.parse(res)
    console.log(imageInfo.value)
    console.log(dayjs(imageInfo.value!.createTime).format('YYYY/MM/DD HH:mm:ss'))
    console.log(dayjs(imageInfo.value!.dateTimeOriginal).format('YYYY/MM/DD HH:mm:ss'))
  })
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
})

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
      <div v-if="imageInfo!.updateTime">
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
