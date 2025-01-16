<script setup lang="ts">
// 切换主题
import { ref } from 'vue'
import {
  Document,
  Menu as IconMenu,
  Location,
  Setting,
  ArrowLeft,
  ArrowRight,
  Folder,
  FolderOpened,
  Files
} from '@element-plus/icons-vue'
import { useRoute } from 'vue-router'


const route = useRoute()
// region tab 项

const isCollapse = ref(true)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
// endregion

</script>

<template>
  <div class="flex w-full h-full flex-col">
    <div class="flex flex-1">
      <!-- 左侧 Tab 栏 -->
      <div class="sticky top-0 flex-shrink-0 border-r p-4 overflow-y-auto h-screen">
        <!-- 控制列表展示状态 -->
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

        <!-- Tab 菜单 -->
        <el-menu
          router
          class="w-full"
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
            </template>
          </el-menu-item>
          <el-menu-item index="/home/folder">
            <el-icon>
              <FolderOpened />
            </el-icon>
            <template #title> 文件夹</template>
          </el-menu-item>
          <el-menu-item index="/home/setting">
            <el-icon>
              <setting />
            </el-icon>
            <template #title> 设置</template>
          </el-menu-item>
          <el-menu-item index="/home/library">
            <el-icon>
              <Files />
            </el-icon>
            <template #title> 资料库</template>
          </el-menu-item>
        </el-menu>
      </div>

      <!-- 右侧内容区 -->
      <div class="flex-1 overflow-auto p-4">
        <router-view />
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
/* 取消 element-plus tab 栏右侧边框展示 */
.el-menu {
  border: none;
}
</style>
