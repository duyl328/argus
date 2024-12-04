/**
 * Time:2024/7/30 下午6:43 25
 * Name:chineseLoremGenerate.test.ts
 * Path:src/__test__/utils
 * ProjectName:utopia-front-vue
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import { describe,it, expect } from 'vitest'
import loremGenerate from '@/utils/loremGenerate'

describe("乱文假数功能测试",() => {
  it('should 文本生成', () => {
    // console.log(ChineseLoremGenerate.generateChinese(100));
    console.log(loremGenerate.generateLoremArticle());
  })
})
