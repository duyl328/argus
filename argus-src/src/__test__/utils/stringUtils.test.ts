/**
 * Time:2024/7/26 下午10:49 28
 * Name:stringUtils.test.ts
 * Path:src/components/__tests__/utils
 * ProjectName:utopia-front-vue
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import { describe, it, beforeEach, afterEach, expect } from 'vitest'

import StringUtils from '@/utils/stringUtils'

describe('String Utils 字符串工具类', () => {
  it('isBlank: 是否为 `null` 或 `空字符串`', () => {
    expect(StringUtils.isBlank('')).toBe(true)
    expect(StringUtils.isBlank(' ')).toBe(true)
    expect(StringUtils.isBlank('123')).toBe(false)
    expect(StringUtils.isBlank(null)).toBe(true)
  })
  it('isEmpty: 此字符序列为空（不包含任何字符），则返回 `true`', () => {
    expect(StringUtils.isEmpty('')).toBe(true)
    expect(StringUtils.isEmpty(' ')).toBe(false)
    expect(StringUtils.isEmpty('123')).toBe(false)
    expect(StringUtils.isEmpty(null)).toBe(true)
  })
  it('toCamelCase: 将字符串转换为小驼峰命名', () => {
    expect(StringUtils.toCamelCase('')).toBe('')
    expect(StringUtils.toCamelCase(' ')).toBe(' ')
    expect(StringUtils.toCamelCase('test')).toBe('test')
    expect(StringUtils.toCamelCase('testTest')).toBe('testTest')
    expect(StringUtils.toCamelCase('TestTest')).toBe('testTest')
    expect(StringUtils.toCamelCase('TestTest123')).toBe('testTest123')
    expect(StringUtils.toCamelCase('TestTest其他字符')).toBe('testTest其他字符')
  })
  it('toPascalCase', () => {
    expect(StringUtils.toPascalCase('')).toBe('')
    expect(StringUtils.toPascalCase(' ')).toBe(' ')
    expect(StringUtils.toPascalCase('test')).toBe('Test')
    expect(StringUtils.toPascalCase('testTest')).toBe('TestTest')
    expect(StringUtils.toPascalCase('TestTest')).toBe('TestTest')
    expect(StringUtils.toPascalCase('TestTest123')).toBe('TestTest123')
    expect(StringUtils.toPascalCase('TestTest其他字符')).
      toBe('TestTest其他字符')
  })
  it('toCustomCase', () => {
    expect(StringUtils.toCustomCase('')).toBe('')
    expect(StringUtils.toCustomCase(' ')).toBe(' ')
    expect(StringUtils.toCustomCase('test')).toBe('test')
    expect(StringUtils.toCustomCase('testTest')).toBe('test-Test')
    expect(StringUtils.toCustomCase('testTest', '+')).toBe('test+Test')
    expect(StringUtils.toCustomCase('TestTest')).toBe('Test-Test')
    expect(StringUtils.toCustomCase('TestTest123')).toBe('Test-Test123')
    expect(StringUtils.toCustomCase('TestTest其他字符')).
      toBe('Test-Test其他字符')
    expect(StringUtils.toCustomCase('SHOWTEST')).
      toBe('SHOWTEST')
  })
  
})


describe('Performance test with hooks', () => {
  const str1 = ''
  const str2 = ''
  
  let start = 0
  beforeEach(() => {
    start = performance.now()
  })
  
  afterEach(() => {
    const end = performance.now()
    const timeTaken = end - start
    console.log(`Execution time: ${timeTaken} ms`)
  })
  
  it('should measure the execution time', () => {
    // 可以添加断言，确保测试逻辑正确
    expect(StringUtils.equals(str1, str2)).toBe(true) // 示例断言
  })
})
