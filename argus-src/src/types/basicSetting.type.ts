/**
 * Time:2024/12/12 10:46 42
 * Name:basicSettingModel.ts
 * Path:src/types
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

/**
 * 基础设置
 */
export type basicSettingType = {
  create_time: bigint
  id: number
  /**
   * 图片路径
   */
  img_paths: string
  update_time: bigint
  path: string
}
