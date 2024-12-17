<script setup lang="ts">
import { disLog, logB, logN, logS } from '@/utils/logHelper/logUtils'

logN.success('埋点', 'page_click_report', {
  eventName: 'page_click_report',
  params: {
    user_id: 'linwu',
    age: '18',
    height: '171cm',
    sex: 'male',
    wx: 'linwu-hi'
  }
})

console.time('代码时间统计')

console.timeLog('代码时间统计')

console.error('错误日志哦！！！')
console.warn('警告日志哦！！！')
console.log('普通日志哦！！！')
console.timeEnd('代码时间统计')

logS.error('1231')
logB.success('测试')
console.count('11111111111')
console.trace()

console.count('11111111111')

</script>

<template>
  <h1>打开控制台查看日志</h1>
</template>

<style scoped>

</style>
