import { invoke } from '@tauri-apps/api/core'
import { getAllPostCommand, insertPostCommand } from '@/constants/command'

/**
 * Time:2024/12/5 10:15 12
 * Name:storageService.ts
 * Path:src/services
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

export function getAllPost() {
  return invoke(getAllPostCommand)
}

export function insertPost(title: string, body: string) {
  invoke(insertPostCommand, { title, body })
}
