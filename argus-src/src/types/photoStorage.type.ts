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
export type photoStorageType = {
  id: number
  img_paths: String
  is_enable: boolean
  is_delete: boolean
  create_time: number
  update_time: number
}
