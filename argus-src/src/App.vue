<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import PathUtils from '@/utils/pathUtils'
import StringUtils from '@/utils/stringUtils'
import type { subRoute, subRouteList } from '@/types/views/dev/DevIndex.type'
import app from '@/constants/app'
import { addListener, emitInit } from '@/services/emits/base'
import { changedTheme, isDark, toggleDark } from '@/utils/darkUtil'
import { onMounted, ref, watch } from 'vue'
import { useDark } from '@vueuse/core'

import { ElNotification } from 'element-plus'
import { h } from 'vue'
import emitOrder from '@/constants/emitOrder'
import { GlobalErrorMsg } from '@/models/globalErrorMsg'
import { logB } from '@/utils/logHelper/logUtils'
import { emitGlobalMsg } from '@/services/base'
import { getAppStatus } from '@/AppStatus'

const router = useRouter()
const isCollapse = ref(true)

// 获取状态实例
let appStatus = getAppStatus()

// 初始化后端监听器
emitInit().then(() => {
  appStatus.emitIsInit.value = true
})

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

function getSwitch() {
  changedTheme()
}

function func() {
  console.log('触发')
  let stringPromise = emitGlobalMsg()
  stringPromise.then((res) => {
    console.log(res)
  })
}

watch(appStatus.emitIsInit, (value, oldValue, onCleanup) => {
  if (value) {
    extracted()
  }
})
if (appStatus.emitIsInit.value){
  if (appStatus.emitIsInit.value) {
    extracted()
  }
}

function extracted() {
  addListener(emitOrder.globalErrorMsgDisplay, (event) => {
    console.log('123123')
    console.log(event)

    let str = event.payload as string
    if (StringUtils.isBlank(str)) return

    let parse: GlobalErrorMsg | null = null
    try {
      parse = JSON.parse(str)
    } catch (e) {
      console.error('后端数据转换出错! ', e)
    }

    // 弹窗
    ElNotification({
      title: parse?.title || '后端信息',
      message: parse?.msg || str,
      position: 'bottom-right',
      type: parse?.type || '',
      duration: parse?.duration
    })
  })
}

</script>

<template>
  <div class="flex flex-col min-h-screen">
    <!-- 顶部导航 -->
    <ul
      class="sticky top-0 shadow-md z-10 flex flex-row items-center p-4 space-x-4"
      v-if="isShowGenerateRouter"
    >
      <li class="list-none" v-for="(subRoute, index) in subRouteLists" :key="index">
        <ElButton :type="isActive(subRoute.path) ? 'primary' : ''" @click="listClickJump(subRoute)">
          {{ subRoute.displayName }}
        </ElButton>
      </li>
      <!-- 顶部功能按钮 -->
      <ElButton class="button" @click="getSwitch">切换主题</ElButton>
      <button @click="func">触发</button>
    </ul>

    <hr v-if="isShowGenerateRouter" />

    <!--  展示主要内容-->
    <div class="flex-1">
      <router-view />
    </div>
  </div>
</template>
<style scoped>
/* 修复高度相关问题，确保 sticky 有效果 */
html,
body,
#app {
  height: 100%;
  margin: 0;
}

/* 使用 tailwindcss */
.button {
  @apply bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-700;
}
</style>
