<script setup lang="ts">
// 切换主题
import { toggleDark } from '@/utils/darkUtil'
import { ref } from 'vue'
import {
  Document,
  Menu as IconMenu,
  Location,
  Setting,
  ArrowLeft,
  ArrowRight
} from '@element-plus/icons-vue'
import { useRoute } from 'vue-router'

function getSwitch() {
  toggleDark()
}

const route = useRoute()
// region tab 项

const isCollapse = ref(true)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <!--    顶部内容【功能区】-->
    <div class="flex-shrink-0 whitespace-nowrap" v-if="false">
      <ElButton @click="getSwitch">切换主题</ElButton>
      <button class="button">你好</button>
    </div>
    <!--    底部主要内容-->
    <div class="flex-1 flex overflow-hidden">
      <!--    左侧 tab 栏展示-->
      <div class="flex-shrink-0 overflow-hidden border-r">
        <!--      控制列表展示状态-->
        <el-radio-group v-model="isCollapse" class="mt-3 w-full" style="margin-bottom: 20px">
          <el-radio-button class="ml-auto mr-2" v-if="isCollapse" :value="false">
            <el-icon>
              <ArrowRight />
            </el-icon>
          </el-radio-button>
          <el-radio-button v-else :value="true" class="ml-auto mr-2">
            <el-icon>
              <ArrowLeft />
            </el-icon>
          </el-radio-button>
        </el-radio-group>

        <!--        tab 展示-->
        <el-menu
          router
          class=""
          :default-active="route.path"
          :collapse="isCollapse"
          @open="handleOpen"
          @close="handleClose"
        >
          <el-sub-menu index="/">
            <template #title>
              <el-icon>
                <location />
              </el-icon>
              <span>Navigator One</span>
            </template>
            <el-menu-item-group>
              <template #title><span>Group One</span></template>
              <el-menu-item index="/">item one</el-menu-item>
              <el-menu-item index="/">item two</el-menu-item>
            </el-menu-item-group>
            <el-menu-item-group title="Group Two">
              <el-menu-item index="/">item three</el-menu-item>
            </el-menu-item-group>
            <el-sub-menu index="/">
              <template #title><span>item four</span></template>
              <el-menu-item index="/">item one</el-menu-item>
            </el-sub-menu>
          </el-sub-menu>
          <el-menu-item index="/">
            <el-icon>
              <icon-menu />
            </el-icon>
            <template #title>Navigator Two</template>
          </el-menu-item>
          <el-menu-item index="/" disabled>
            <el-icon>
              <document />
            </el-icon>
            <template #title>
              <span>Navigator Three</span>
              <!--              <span class="ml-12 mr-12">500</span>-->
            </template>
          </el-menu-item>
          <el-menu-item index="/home/setting">
            <el-icon>
              <setting />
            </el-icon>
            <template #title> 设置</template>
          </el-menu-item>
          <el-menu-item index="/home/library">
            <el-icon>
              <setting />
            </el-icon>
            <template #title> 资料库</template>
          </el-menu-item>
        </el-menu>
      </div>
      <!--    右侧主要内容展示-->
      <main class="flex-1 overflow-hidden">
        <router-view />
      </main>
    </div>
  </div>
</template>

<style scoped lang="scss">
// 取消 element-plus tab 栏右侧边框展示
.el-menu {
  border: none;
}

// 使用 tailwindcss
.button {
  @apply bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-700;
}
</style>
