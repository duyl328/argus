<script setup lang="ts">
import { RouterView, useRoute, useRouter } from 'vue-router'
import PathUtils from '@/utils/pathUtils'
import type { subRoute, subRouteList } from '@/types/views/dev/DevIndex.type'
import StringUtils from '@/utils/stringUtils'
import ArrayUtils from '@/utils/arrayUtils'
import { onMounted } from 'vue'
import gsap from 'gsap'
import { TextPlugin } from 'gsap/TextPlugin'
import { computed } from 'vue'

gsap.registerPlugin(TextPlugin)

const router = useRouter()
const route = useRoute()

const routes = import.meta.glob('@/views/dev/**/*.vue')

// region 子路由列表 点击、激活 处理
// 子路由列表
let subRouteLists: subRouteList = []

// 遍历路由page列表进行路由渲染
Object.keys(routes).map((path) => {
  if ('.vue' === PathUtils.extname(path).trim()) {
    let fileName = PathUtils.basename(path).split('.')[0]
    let items = {
      displayName: fileName,
      path: '/dev-index' + `/${StringUtils.toCustomCase(fileName).toLowerCase()}` // 生成路径
    }
    subRouteLists.push(
      items
    )
  }
  return ''
})

/**
 * 列表点击调准
 */
function listClickJump (item: subRoute) {
  console.log(item.path);
  router.push({ path: item.path })
}

/**
 * 列表路径是否激活
 * @param path
 */
const isActive = (path: string) => {
  return route.path.includes(path)
}
// endregion

// region 无子路由数据自动跳转至主页
// 多久后跳转
let countdownSeconds = 9
// 指定跳转路径
const targetUrl = '/'
// 跳转定时器
let targetCountdown: NodeJS.Timeout
// 路由列表是否为空【计算属性存在缓存】
const routerListIsEmpty = computed(() => {
  return ArrayUtils.isEmpty(subRouteLists)
})

onMounted(() => {
  // 如果数据为空，设置跳转倒计时
  if (routerListIsEmpty.value) {
    targetCountdown = setInterval(() => {
      gsap.fromTo('#animated-text',
        { text: countdownSeconds.toString(), opacity: 1, y: -10 },
        {
          duration: 1,
          y: 150,
          text: (countdownSeconds--).toString(),
          ease: 'expo.in'
        }
      )
      if (countdownSeconds < 0) {
        jumpToHome()
      }
    }, 1500)
  }
})

/**
 * 跳转至主页
 */
function jumpToHome () {
  clearInterval(targetCountdown)
  router.replace({ path: targetUrl })
}

// endregion

</script>

<template>
  <div class="h-screen select-none">
    <h1
      class="text-blue-900 font-bold font-sans text-5xl text-center mt-10 mb-10">
      Dev
    </h1>
    <hr>

    <div v-if="routerListIsEmpty" class="bg-slate-50 overflow-hidden ">
      <div
        class="text-center content-center pb-80 font-serif  text-red-500 text-6xl  ">
        无数据!
        <br>
        <div
          class="w-auto -translate-x-80 translate-y-10 inline-flex text-2xl  min-h-40 h-40 max-h-40 overflow-hidden text-blue-900  justify-center items-center content-center">
          <div class="top text-center content-center h-40">
            <p id="animated-text" class="text-center content-center text-4xl">9</p>
          </div>
          <p class="">
            <span>秒后跳转至</span>
            <span class="ml-1.5 underline cursor-pointer" @click="jumpToHome">主页</span>
          </p>
        </div>
      </div>
    </div>
    <div v-else class="flex mt-5 flex-column m-auto lg:w-9/12">
      <ul class="mr-20 hidden lg:block">
        <li
          class="list-none  list-item p-1 m-1 transition-all select-none cursor-pointer
            hover:text-yellow-300 text-gray-700  "
          v-for="(subRoute,index) in subRouteLists"
          :class="{
            'font-bold underline text-2xl': isActive(subRoute.path),
          }"
          :key="index"
          @click="listClickJump(subRoute)">
          {{ subRoute.displayName }}
          <!--          <span-->
          <!--            class=" line-clamp-2 max-h-14 text-wrap p-1">-->
          <!--            {{ subRoute.displayName }}-->
          <!--          </span>-->
        </li>
      </ul>
      <main class="content flex mt-5 overflow-y-auto flex-column m-auto">
        <router-view />
      </main>
    </div>
  </div>

</template>

<style scoped>
.content {
  width: calc(100% - 10rem);
  border: #f1f1f1 solid 1px;
  padding: 5px;
  border-radius: 5px;
}


</style>
