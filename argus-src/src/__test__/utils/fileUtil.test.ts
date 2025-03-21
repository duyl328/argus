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
import { convertFileSize } from '@/utils/fileUtil'
import { FILE_SIZE_UNIT_ENUM } from '@/constants/enums'

describe('constants.ts', () => {
  it('should ', () => {
    expect(() => {
      convertFileSize(-1, FILE_SIZE_UNIT_ENUM.B)
    }).toThrow()
  })
  it('数据单位选择', () => {
    console.log(convertFileSize(0, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(100, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(10000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(100000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(10000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(100000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(10000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(100000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(10000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(100000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1000000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(10000000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(100000000000000000, FILE_SIZE_UNIT_ENUM.B))
    console.log(convertFileSize(1000000000000000000, FILE_SIZE_UNIT_ENUM.B))
  })
})
