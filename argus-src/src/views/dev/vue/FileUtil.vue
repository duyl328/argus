<template>
  <CommandManager :pro="commands"></CommandManager>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { baseInvoke } from '@/utils/commandUtil.js'
import CommandManager from '@/components/dev/CommandManager.vue'
import type { CommandType } from '@/types/command.type' // 引入类型定义

let newVar: CommandType = {
  name: 'greet',
  description: '基础指令测试，tauri默认指令，输入名字会欢迎你!',
  params: [
    {
      // 参数名称
      name: 'name',
      label: '输入名称',
      type: 'text',
      value: '',
      placeholder: '请输入数据'
    }
  ],
  result: null
}
const commands: CommandType[] = [
  newVar,
  {
    name: 'get_all_sub_dir',
    description: '获取所有子文件夹的测试',
    params: [
      {
        name: 'path',
        label: '文件路径',
        type: 'text',
        value: 'D:\\argus\\img',
        placeholder: '文件路径'
      }
    ],
    result: null
  },
  {
    name: 'get_all_imgs',
    description: '获取所有子文件夹的测试',
    params: [
      {
        name: 'path',
        label: '文件路径',
        type: 'text',
        value: 'D:\\argus\\img\\局部',
        placeholder: '文件路径'
      }
    ],
    result: null
  },
  {
    name: 'get_dir_all_subfolders_first_img',
    description: '获取指定路径所有子路径的第一张图片，没有图片则不返回内容',
    params: [
      {
        name: 'path',
        label: '文件路径',
        type: 'text',
        value: 'D:\\argus\\img',
        placeholder: '文件路径'
      }
    ],
    result: null
  },

]
</script>

<style></style>
