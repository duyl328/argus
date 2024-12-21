/**
 * Time:2024/12/20 20:02 41
 * Name:imageService.ts
 * Path:src/services
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import {invoke} from "@tauri-apps/api/core";
import {
  generateSaveThumbnailCommand,
  getImageAbsolutePathCommand,
  readImageAsBase64Command
} from '@/constants/command'

/**
 * 生成保存缩略图
 */
export  function generateSaveThumbnail (imagePath:string[], emitCommand:string) {
  return invoke<string[]>(generateSaveThumbnailCommand,{imagePath, emitCommand});
}
