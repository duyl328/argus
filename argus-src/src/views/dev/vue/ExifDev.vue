<script setup lang="ts">
import CommandManager from '@/components/dev/CommandManager.vue'
import type { CommandType } from '@/types/command.type'

const commands: CommandType[] = [
  {
    name: 'get_exif_info',
    description: '查询exif信息',
    params: [
      {
        name: 'path',
        label: 'path',
        type: 'text',
        value: 'D:\\argus\\argus-src\\src-tauri\\resources\\image\\image-1-1.JPG',
        placeholder: 'string'
      }
    ],
    result: null
  }
]
</script>

<template>
  <CommandManager :pro="commands"></CommandManager>
</template>

<style scoped></style>
