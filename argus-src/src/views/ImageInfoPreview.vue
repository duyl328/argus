<script setup lang="ts">
import { Window } from '@tauri-apps/api/window'
import { ref, computed, onBeforeUnmount } from 'vue'

// 接收关闭状态
import { onMounted } from 'vue'
import type { ImageShowInfo } from '@/models/ImageShowInfo'
import { getImageInfo } from '@/services/imageService'
import type { ImageInfo } from '@/types/image.type'
import ImagePreview from '@/views/ImagePreview.vue'

// 接收关闭事件
const props = defineProps({
  imgInfo: Object as () => ImageShowInfo,
  closePreview: Function
})
console.log(props.imgInfo)
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
    @click.prevent
    @blur.prevent
    @abort.prevent
    @scroll.prevent
    @drag.prevent
    @focus.prevent
    @mousedown.prevent
    @click.stop
    class="fixed inset-0 z-50 flex bg-black bg-opacity-90"
  >
    <ImagePreview class="flex-1" :imgInfo="previewImage" />
    <div class="w-96 h-full bg-red-600">

      <!--  重置图片位置 -->
      <!--    <el-button-->
      <!--      v-if="!isInViewport"-->
      <!--      @click="resetImagePosition"-->
      <!--      class="absolute bottom-10 left-1/2 transform -translate-x-1/2 -translate-y-1/2"-->
      <!--    >重置图片-->
      <!--    </el-button>-->

      <!--
      上一个
      下一个
      放大
      缩小
      旋转（左右）
      镜像（待定）
      -->

      <!-- 操作菜单 -->
      <div class="w-full h-full bg-green-200 p-4" :class="isShowInfo ? 'block' : 'hidden'">
        <el-tabs class="demo-tabs">
          <el-tab-pane label="User" name="first">User</el-tab-pane>
          <el-tab-pane label="Config" name="second">Config</el-tab-pane>
          <el-tab-pane label="Role" name="third">Role</el-tab-pane>
          <el-tab-pane label="Task" name="fourth">Task</el-tab-pane>
        </el-tabs>
        基础信息 HASH、 文件名、日期、时区、位置（海拔、经纬度）、拍摄设备、
        ISO、曝光时间、光圈、焦距、软件版本、评分、相机制造商、相机型号、软件版本、闪光灯
        创建日期、最大光圈值、曝光程序、测光模式、艺术家、标签 人物 文件
        路径、文件名称、HASH、宽高、文件大小、图像比例、图片格式 其他
        是否经过算法、算法评分、上次查看时间、创建日期、更新时间
      </div>

      <!-- 关闭按钮 -->
      <button
        @click="closePreview"
        class="absolute top-12 right-12 bg-white text-black rounded-full p-2 pl-3.5 pr-3.5 hover:bg-gray-300"
      >
        ✖
      </button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.img-transition {
  transition: all 0.2s;
}
</style>
