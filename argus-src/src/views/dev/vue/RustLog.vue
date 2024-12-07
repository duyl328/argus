<script setup lang="ts">
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log'
import { logLogs } from '@/services/logService'

trace('Trace')
info('Info')
error('Error')

function log() {
  logLogs()
}
</script>

<template>
  <h1>rust log输出</h1>
  <el-button @click="log">打印日志</el-button>
</template>

<style scoped></style>
