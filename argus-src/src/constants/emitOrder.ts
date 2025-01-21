/**
 * Time:2024/12/17 21:34 26
 * Name:emitOrder.ts
 * Path:src/constants
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

const EmitOrder = {
  /**
   * 下载开始
   */
  downloadStartedCommand: 'download-started',

  /**
   * 文件夹展示 - 图像压缩完成
   */
  folderViewImageShow: 'folder-view-image-show',

  /**
   * 全局报错提示
   */
  globalErrorMsgDisplay: 'global-error-msg-display',

  /**
   * 照片后台加载报错提示
   */
  photoLoadingErrTip: 'photo-loading-err-tip',

  /**
   * 照片后台加载进度及信息提示
   */
  photoLoadingMsgTip: 'photo-loading-msg-tip'
} as const

export default EmitOrder
