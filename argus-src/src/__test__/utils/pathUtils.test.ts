/**
 * Time:2024/7/27 上午11:46 37
 * Name:pathUtils.test.ts
 * Path:src/__test__/utils
 * ProjectName:utopia-front-vue
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

import { describe, it, expect } from 'vitest'
import PathUtils from '@/utils/pathUtils'

describe('路径工具类测试', () => {
  it('路径拼接', () => {
    expect(PathUtils.join()).toBe('')
    expect(PathUtils.join('', '', '')).toBe('')
    expect(PathUtils.join('C:', 'system', 'window')).toBe('C:/system/window')
    expect(PathUtils.join('C:', 'system', 'window', 'show.txt')).
      toBe('C:/system/window/show.txt')
    expect(PathUtils.join('C:', 'system', '/window/office', 'show.txt')).
      toBe('C:/system/window/office/show.txt')
  })
  it('返回路径的目录名称.', () => {
    expect(PathUtils.dirname('')).toBe('')
    expect(PathUtils.dirname(' ')).toBe(' ')
    expect(PathUtils.dirname('C:/system/window/show.txt')).toBe('C:/system/window')
    expect(PathUtils.dirname('/system/window/show.txt')).toBe('/system/window')
  })
  
  it('路径的扩展名.', () => {
    expect(PathUtils.extname('')).toBe('')
    expect(PathUtils.extname(' ')).toBe('')
    expect(PathUtils.extname('C:/system/window/show.txt')).toBe('.txt')
    expect(PathUtils.extname('/system/window/show.txt')).toBe('.txt')
    expect(PathUtils.extname('/system/window/show.txt.mp3')).toBe('.txt.mp3')
    expect(PathUtils.extname('system/window/show.txt.mp3')).toBe('.txt.mp3')
    expect(PathUtils.extname('system/window/show')).toBe('')
  })

  it('返回路径的最后一部分.', () => {
    expect(PathUtils.basename('')).toBe('')
    expect(PathUtils.basename(' ')).toBe(' ')
    expect(PathUtils.basename('C:/system/window/show.txt')).toBe('show.txt')
    expect(PathUtils.basename('/system/window/show.txt')).toBe('show.txt')
    expect(PathUtils.basename('/system/window/show.txt.mp3')).toBe('show.txt.mp3')
    expect(PathUtils.basename('system/window/show.txt.mp3')).toBe('show.txt.mp3')
    expect(PathUtils.basename('system/window/show')).toBe('show')
  })
})
