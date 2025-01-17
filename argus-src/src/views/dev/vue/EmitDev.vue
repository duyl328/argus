<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import CommandManager from '@/components/dev/CommandManager.vue'
import type { CommandType } from '@/types/command.type'
import { addListener, clearListeners } from '@/services/emits/base'
import emitOrder from '@/constants/emitOrder'
import { GlobalErrorMsg } from '@/models/globalErrorMsg'
import { ElNotification } from 'element-plus'
import { watch, type WatchStopHandle } from 'vue'
import { getAppStatus } from '@/AppStatus'

// 注册监听器，统一处理图片数据
// async function startImageListener() {
//   await listen('download-started', (event) => {
//     const batch = event.payload // 批量图片数据
//     console.log(batch)
//   })
// }
let appStatus = getAppStatus()

let stop: WatchStopHandle|null = null;
stop = watch(appStatus.emitIsInit, (value, oldValue, onCleanup) => {
  if (value) {
    extracted()
    if (stop != null){
      stop()
    }
  }
}, {
  immediate: true
})

function extracted() {
  addListener(emitOrder.downloadStartedCommand, (event) => {
    console.log(event)
    console.log(event.payload)
  })
}


const commands: CommandType[] = [
  {
    name: 'emit_send_test',
    description: 'emit 触发测试',
    params: [
      {
        name: 'param',
        label: '测试参数（字符串）',
        type: 'text',
        value: '世界',
        placeholder: ''
      }
    ],
    result: null
  }, {
    name: 'emit_global_msg',
    description: '全局 emit 测试报错',
    params: [],
    result: null
  },
  {
    name: 'global_msg_emit',
    description: '全局 emit 报错测试',
    params: [],
    result: null
  }
]
</script>

<template>
  <h1>Emit</h1>
  <CommandManager :pro="commands"></CommandManager>
</template>

<style scoped></style>
