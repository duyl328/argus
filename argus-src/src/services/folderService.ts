import { invoke } from '@tauri-apps/api/core'
import {
  getDirAllSubfoldersFirstImgCommand,
  getNeedDisplayImageInfoCommand,
  insertPostCommand
} from '@/constants/command'
import type { FolderImage } from '@/types/rusts/FolderImage'

/**
 * Time:2024/12/14 15:55 56
 * Name:folderService.ts
 * Path:src/services
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

/**
 * 获取指定路径所有子路径的第一张图片，没有图片则不返回内容
 */
export function getDirAllSubfoldersFirstImg(path: string,width:number,height:number) {
  return invoke<FolderImage[]>(getDirAllSubfoldersFirstImgCommand, { path ,width ,height })
}

/**
 * 获取文件夹目录需要展示的图片和信息
 * @param path
 */
export function getNeedDisplayImageInfo(path: string) {
  return invoke<FolderImage[]>(getNeedDisplayImageInfoCommand, { path  })
}


/*
















*/
