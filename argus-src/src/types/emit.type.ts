import type { Event } from '@tauri-apps/api/event'
import EmitOrder from '@/constants/emitOrder'

/**
 * Time:2024/12/18 09:04 27
 * Name:emit.type.ts
 * Path:src/types
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
export type EmitListenerType = (event: Event<unknown>) => void
export type EmitListenerArrayType = { [key: string]: EmitListenerType[] }
export type EmitOrderTypes = (typeof EmitOrder)[keyof typeof EmitOrder]
