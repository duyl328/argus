import { invoke } from '@tauri-apps/api/core'
import type { ImageDirRustInfo } from '@/models/ImageShowInfo'
import { addPhotoRetrieveTaskCommand } from '@/constants/command'



/**
 * 添加图像处理任务
 */
export  function addPhotoRetrieveTask (path:string) {
  return invoke<string[]>(addPhotoRetrieveTaskCommand,{path});
}