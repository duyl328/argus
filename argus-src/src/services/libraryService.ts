/**
 * Time:2024/12/12 21:30 54
 * Name:libraryService.ts
 * Path:src/services
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

import { invoke } from '@tauri-apps/api/core'
import { addPhotoStorageCommand, getPhotoStorageCommand, logLogsCommand } from '@/command'
import type { photoStorageType } from '@/types/photoStorage.type'

/**
 * 获取所有图像路径
 */
export function getAllLibrary() {
  return invoke<string>(getPhotoStorageCommand)
}

/**
 * 添加一个地址
 */
export function addPhotoStorage(imgPath: photoStorageType) {
  console.log(imgPath);
  return invoke<string>(addPhotoStorageCommand, { imgath: imgPath })
}
