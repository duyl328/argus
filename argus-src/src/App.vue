<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import PathUtils from '@/utils/pathUtils'
import StringUtils from '@/utils/stringUtils'
import type { subRoute, subRouteList } from '@/types/views/dev/DevIndex.type'
import app from '@/constants/app'
import { ref } from 'vue'

const router = useRouter()

// 是否展示生成路由
const isShowGenerateRouter = ref(false)

// region 获取路由列表
// 自动获取路由列表
const routes = {
  ...import.meta.glob('@/views/*.vue'),
  ...import.meta.glob('@/views/dev/*.vue')
}
const route = useRoute()

// 子路由列表
let subRouteLists: subRouteList = []
// 遍历路由page列表进行路由渲染
Object.keys(routes).map((path) => {
  if ('.vue' === PathUtils.extname(path).trim()) {
    let fileName = PathUtils.basename(path).split('.')[0]
    let s = StringUtils.toCustomCase(fileName).toLowerCase().split('-')[0]
    let items = {
      displayName: fileName,
      path: `/${s}` // 生成路径
    }
    subRouteLists.push(items)
  }
  return ''
})

/**
 * 列表路径是否激活
 * @param path
 */
const isActive = (path: string) => {
  return route.path.includes(path)
}

/**
 * 列表点击调准
 */
function listClickJump(item: subRoute) {
  console.log(item.path)
  router.push({ path: item.path })
}

// endregion

// region 开发和生产状态确认
let nodeenv: string | undefined = process.env.NODE_ENV
console.log(nodeenv)
if (nodeenv !== undefined && !StringUtils.isBlank(nodeenv) && nodeenv === app.DEVELOPMENT) {
  app.IS_PRODUCTION = false
  app.IS_SHOW_GENERATE_ROUTER = true
  isShowGenerateRouter.value = app.IS_SHOW_GENERATE_ROUTER
}
// endregion
</script>

<template>
  <div class="overflow-hidden flex flex-col h-dvh">
    <!--    路由导航-->
    <ul class="mr-20 top-0 sticky hidden lg:flex" v-if="isShowGenerateRouter">
      <li
        class="list-none list-item p-1 m-1 transition-all select-none cursor-pointer hover:text-yellow-300 text-gray-700"
        v-for="(subRoute, index) in subRouteLists"
        :key="index"
      >
        <ElButton :type="isActive(subRoute.path) ? 'primary' : ''" @click="listClickJump(subRoute)">
          {{ subRoute.displayName }}
        </ElButton>
      </li>
    </ul>

    <hr v-if="isShowGenerateRouter" />

    <!--  展示主要内容-->
    <div class="overflow-y-auto flex-1">
      <router-view />
    </div>
  </div>
</template>
