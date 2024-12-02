<script setup lang="ts">
import PathUtils from '../../../utils/pathUtils'
import type { FontStyleObject } from '../../../types/views/dev/font/SupportedFonts.type'
// import PathUtils from "../../../utils/pathUtils.ts"
/**
 * 将需要展示的文字文件放至 `/assets/fonts` ，该组件可以通过文件遍历的方式将所有字体文件找到
 * 并生成对应的样式文件以及 font-face 通过中文和英文模板内容在页面展示
 *
 * 展示逻辑：
 *  1. 读取所有需要的文件 `import.meta.glob` 是 Vite 独有内容
 *  2. 通过遍历所有的字体文件，并使用 `v-for` 生成对应的 html 文件，并绑定样式内容
 *  3. 通过手动常见 style 的方式将内容创建并添加至 html 中，展示对应样式
 */

const fonts = import.meta.glob('@/assets/fonts/**/*.{ttf,otf}')

// 字体样式表
const fontsList: FontStyleObject[] = []

Object.keys(fonts).forEach((key: string) => {
  let font = fonts[key]
  let fontFamily = PathUtils.basename(font.name).split('.')[0]

  // 创建一个新的<style>元素
  const style = document.createElement('style')
  style.textContent = `
        @font-face {
          font-family: '${fontFamily}';
          src: url('${font.name}') format('truetype');
          font-weight: normal;
          font-style: normal;
        }
      `
  // 将<style>元素添加到<head>中
  document.head.appendChild(style)

  fontsList.push({ name: fontFamily, fontStyle: font.name })
})


</script>

<template>
  <div>
    <div
      class="text-3xl text-black  "
      v-for="(item,index) in fontsList"
      :key="index"
      :style="{fontFamily:item.name}"
      style="{item.fontStyle}"
    >
      <br>
      <hr>
      <br>
      <p>
        123456789 ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz
        <span>
      If you can’t fly, then run; if you can’t run, then walk; if you can’t walk, then crawl; but whatever you do, you
      have to keep moving forward.
      </span>
        <br> <span>
        字体内容： {{ item.name }}
      </span><br>
        <span>
        鉴于对人权的无视和侮蔑已发展为野蛮暴行,这些暴行玷污了人类的良心,而一个人人享有言论和信仰自由并免予恐惧和匮乏的世界的来临,已被宣布为普通人民的最高愿望
      </span>
      </p>

    </div>
  </div>
</template>

<style scoped lang="scss">
</style>
