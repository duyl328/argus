/**
 * 全局报错 接收 解析 对象
 */
export class GlobalErrorMsg {
  /**
   * 标题
   */
  title: string
  /**
   * 展示消息
   */
  msg: string
  /**
   * 展示时间【为 0 时则不会自动关闭】
   */
  duration: number

  /**
   * 通知类型
   */
  type: 'success' | 'warning' | 'info' | 'error' | ''

  constructor(title: string, msg: string, duration: number, type: 'success' | 'warning' | 'info' | 'error' | '') {
    this.title = title
    this.msg = msg
    this.duration = duration
    this.type = type
  }
}

/**
 * 进度信息提示
 */
export type loadMsg = {
  // 总任务数
  allTask: number,
  // 当前任务数（已完成）
  currentTask: number,
  // 任务信息
  taskMsg: string,

}