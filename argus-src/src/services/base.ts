import {invoke} from "@tauri-apps/api/core";

import { emitGlobalMsgCommand, greetCommand } from '@/constants/command'

/**
 * 问候
 * @param name
 */
export function greet(name: string) {
    return invoke<string>(greetCommand, {name});
}

/**
 * 后端 emit 测试
 */
export function emitGlobalMsg() {
    return invoke<string>(emitGlobalMsgCommand,{});
}
