<script setup lang="ts">

import CommandManager from '@/components/dev/CommandManager.vue'
import type { CommandType } from '@/types/command.type'

const commands: CommandType[] = [
  {
    name: 'http_example',
    description: '发送请求',
    params: [
    ],
    result: null
  }
]
</script>

<template>
  <CommandManager :pro="commands"></CommandManager>

</template>

<style scoped>

</style>
