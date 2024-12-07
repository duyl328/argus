/**
 * Time:2024/12/7 11:20 01
 * Name:logService.ts
 * Path:src/services
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import { invoke } from '@tauri-apps/api/core'
import { logLogsCommand } from '@/command'

export function logLogs() {
  invoke(logLogsCommand)
}
