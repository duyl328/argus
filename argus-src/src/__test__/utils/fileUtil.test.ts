/**
 * Time:2025/3/19 23:28 25
 * Name:fileUtil.test.ts
 * Path:src/__test__/utils
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import { describe, expect, it, test } from 'vitest'
import { formatFileSize } from '@/utils/fileUtil'
import { FILE_SIZE_UNIT_ENUM } from '@/constants/enums'

describe('constants.ts', () => {
  it('should ', () => {
    expect(() => {
      formatFileSize(-1, FILE_SIZE_UNIT_ENUM.B)
    }).toThrow()
  })
  it('数据单位选择', () => {
    console.log(formatFileSize(0, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(100, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(10000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(100000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(10000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(100000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(10000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(100000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(10000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(100000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1000000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(10000000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(100000000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(formatFileSize(1000000000000000000, FILE_SIZE_UNIT_ENUM.B))
  })
})
