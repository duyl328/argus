/**
 * Time:2025/3/19 22:27 34
 * Name:constants.test.ts
 * Path:src/__test__/utils
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

import { FILE_SIZE_UNIT_SIZE } from '@/constants/constants'

import { describe, it, expect } from 'vitest'

describe('constants.ts', () => {
  it('数据测试', () => {
    Object.values(FILE_SIZE_UNIT_SIZE).forEach((item) => {
      console.log(item.bytes)
    })
  })
})
