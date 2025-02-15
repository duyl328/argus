/**
 * Time:2024/12/22 21:41 04
 * Name:ImageShowInfo.ts
 * Path:src/models
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import { ref } from 'vue'

/**
 * 图片展示所需的全部信息
 */
export class ImageShowInfo {
  isLoading = true
  isError = false
  errorMsg = ''
  // 原图路径
  sourceFilePath = ''
  // 原图展示路径
  sourceFileShowPath = ''
  // 文件路径
  filePath = ''
  // 文件夹路径
  folderPath = ''
  // 缩略图路径
  compressedPath = ''
  public isWhether: boolean = false

  constructor(
    sourceFilePath = '',
    folderPath = '',
    sourceFileShowPath = '',
    filePath = '',
    isLoading: boolean = true,
    isError: boolean = false,
    errorMsg: string = ''
  ) {
    this.sourceFilePath = sourceFilePath
    this.sourceFileShowPath = sourceFileShowPath
    this.filePath = filePath
    this.folderPath = folderPath
    this.isLoading = isLoading
    this.isError = isError
    this.errorMsg = errorMsg
  }
}

/**
 * 后端返回数据
 */
export type ImageDirRustInfo = {
  // 原图路径
  source_file_path: string
  // 文件路径
  file_path: string
}
