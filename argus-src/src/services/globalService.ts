import { getDirAllSubfoldersFirstImgCommand, setAppHandleCommand } from '@/constants/command'
import { invoke } from '@tauri-apps/api/core'
import type { ImageDirRustInfo } from '@/models/ImageShowInfo'


/**
 * 初始化 app handle
 */
export function setAppHandle() {
  return invoke<string>(setAppHandleCommand)
}
