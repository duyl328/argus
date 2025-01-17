import { ref, type Ref } from 'vue'

class AppStatus {
  /**
   * emit 是否已初始化
   */
  emitIsInit: Ref<boolean>

  constructor() {
    this.emitIsInit = ref(false)
  }
}

/**
 * 唯一实例
 */
let _appStatus: AppStatus

/**
 * 获取软件状态
 */
export function getAppStatus() {
  if (_appStatus === null || _appStatus === undefined) {
    _appStatus = new AppStatus()
  }
  return _appStatus
}
