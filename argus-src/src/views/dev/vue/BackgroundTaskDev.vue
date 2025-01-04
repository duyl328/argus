<script setup lang="ts">
import CommandManager from '@/components/dev/CommandManager.vue'
import type { CommandType } from '@/types/command.type'

const commands: CommandType[] = [
  {
    name: 'add_task',
    description: '添加任务',
    params: [
      {
        name: 'task',
        label: 'task',
        type: 'text',
        // 默认值
        value: '添加任务',
        // 提示字段，如：【输入名称】
        placeholder: 'string'
      }
    ],
    result: null
  },
]
</script>

<template>
  <CommandManager :pro="commands"></CommandManager>
</template>

<style scoped></style>
