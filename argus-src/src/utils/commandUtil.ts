import { invoke } from '@tauri-apps/api/core'
import { greetCommand } from '@/constants/command'

/**
 * Time:2024/12/14 12:21 30
 * Name:commandUtil.ts
 * Path:src/utils
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
export function baseInvoke<T>(command: string, args: any) {
  return invoke<T>(command, args)
}
