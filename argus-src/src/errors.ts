/**
 * Time:2025/4/7 15:27 47
 * Name:errors.ts
 * Path:src
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

import logUtil from '@/constants/logUtil'
import { logN } from '@/utils/logHelper/logUtils'

/**
 * 拓展异常类
 */
export class ExpandError extends Error {
  /**
   * 错误信息
   */
  message: string = ''
  /**
   * 错误名称
   */
  name: string = ''

  /**
   * 建议信息
   */
  suggest: string = ''
  /**
   * 错误代码
   */
  errorCode: number = -1
  /**
   * 是否放行错误
   */
  errorIsGoon: boolean = false

  /**
   * 构造函数
   * @param message 错误信息
   * @param name 错误名称
   * @param suggest 建议信息
   * @param errorCode 错误码
   * @param errorIsGoon 是否放行错误
   */
  constructor(
    message: string = '',
    name: string = '',
    suggest: string = '',
    errorCode: number = -1,
    errorIsGoon: boolean = false
  ) {
    super(message)
    this.name = name
    this.message = message
    this.suggest = suggest
    this.errorCode = errorCode
    this.errorIsGoon = errorIsGoon
  }

  log() {
    const message = `log: 定义异常 错误码：[${this.errorCode}], 错误信息：[${this.name}] ,错误信息：[${this.message}] ,建议：[${this.suggest}]`
    console.error(message)
  }

  // region 图片预览错误
  /**
   * 已是最后一张
   */
  static IsLastOneError = new ExpandError('已是最后一张', 'IsLastOne', '', 100001, false)

  /**
   * 已是第一张
   */
  static IsFirstOneError = new ExpandError('已是第一张', 'IsFirstOne', '', 100002, false)

  // endregion
}
