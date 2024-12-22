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
  imgPath = ''
  compressedPath = ''
  public isWhether: boolean = false

  constructor(
    imgPath: string,
    isLoading: boolean = true,
    isError: boolean = false,
    errorMsg: string = ''
  ) {
    this.isLoading = isLoading
    this.isError = isError
    this.errorMsg = errorMsg
    this.imgPath = imgPath
  }
}
