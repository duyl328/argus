import { invoke } from '@tauri-apps/api/core'
import { getAllImgsCommand, getDirAllSubfoldersFirstImgCommand, insertPostCommand } from '@/constants/command'
import type { FolderImage } from '@/types/rusts/FolderImage'
import type { ImageDirRustInfo } from '@/models/ImageShowInfo'

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
export function getDirAllSubfoldersFirstImg(path: string) {
  return invoke<ImageDirRustInfo[]>(getDirAllSubfoldersFirstImgCommand, { path })
}

/**
 * 获取指定路径下所有文件
 */
export function getAllImgs(path: string){
  return invoke<ImageDirRustInfo[]>(getAllImgsCommand, { path })
}


/*
















*/
