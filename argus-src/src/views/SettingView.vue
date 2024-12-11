<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>

<template>
  <div class="flex w-full h-full flex-col m-1">
    <!--  顶部 tab 展示-->
    <div class="flex-shrink-0 whitespace-nowrap">
      <el-menu
        router
        :default-active="route.path"
        class="el-menu-demo"
        mode="horizontal"
        @select="handleSelect"
      >
        <el-menu-item index="/home/setting/basic-setting">一般</el-menu-item>
        <el-menu-item index="/">Processing Center2</el-menu-item>
        <el-menu-item index="/">Processing Center2</el-menu-item>
        <el-menu-item index="/">Processing Center2</el-menu-item>
        <!--        <el-menu-item class="flex-1 justify-between" index="/">Processing Center3</el-menu-item>-->
      </el-menu>
    </div>
    <!--  内容展示-->
    <div class="flex-1 flex overflow-auto">
      <router-view />
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
