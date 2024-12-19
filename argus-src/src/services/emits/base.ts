/**
 * Time:2024/12/17 21:12 53
 * Name:base.ts
 * Path:src/services/emits
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import { type Event, listen } from '@tauri-apps/api/event'
import EmitOrder from '@/constants/emitOrder'
import type { EmitListenerArrayType, EmitListenerType, EmitOrderTypes } from '@/types/emit.type'

/**
 * emit 函数数组
 */
let emitFuncList: EmitListenerArrayType = {}

/**
 * 添加监听器
 * @param type
 * @param listener
 */
export function addListener(type: EmitOrderTypes, listener: EmitListenerType) {
  if (!emitFuncList[type]) {
    emitFuncList[type] = []
  }

  emitFuncList[type].push(listener)
}

/**
 * 初始化
 */
export async function emitInit() {
  for (let emitOrderKey in EmitOrder) {
    const key = emitOrderKey as keyof typeof EmitOrder; // 显式转换类型
    let emitOrderElement = EmitOrder[key]
    await listen(emitOrderElement, (event) => {
      emit(emitOrderElement, event)
    })
  }
  emitFuncList = {}
}

/**
 * 移除指定类型的某个监听器
 * @param type
 * @param listener
 */
export function removeListener(type: string, listener: EmitListenerType): void {
  if (!emitFuncList[type]) return

  const index = emitFuncList[type].indexOf(listener)
  if (index !== -1) {
    emitFuncList[type].splice(index, 1)
  }
}

// 清空指定类型的所有监听器
export function clearListeners(type: string): void {
  if (emitFuncList[type]) {
    emitFuncList[type] = []
  }
}

// 触发指定类型的所有监听器
function emit(type: string, args: any): void {
  if (!emitFuncList[type]) return

  // 依次执行所有监听器
  for (const listener of emitFuncList[type]) {
    listener(args)
  }
}
