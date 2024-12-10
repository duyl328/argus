<script setup lang="ts">
import { ref } from 'vue'
import type { TabsInstance } from 'element-plus'

// 切换主题
import { toggleDark } from '@/utils/darkUtil'

function getSwitch() {
  toggleDark()
}

// region tab 项
const tabPosition = ref<TabsInstance['tabPosition']>('left')

// endregion
</script>

<template>
  <main>
    <ElButton @click="getSwitch">切换主题</ElButton>
    <el-tabs :tab-position="tabPosition" style="height: 200px" class="demo-tabs">
      <el-tab-pane label="User">User</el-tab-pane>
      <el-tab-pane label="Config">Config</el-tab-pane>
      <el-tab-pane label="Role">Role</el-tab-pane>
      <el-tab-pane label="Task">Task</el-tab-pane>
    </el-tabs>
  </main>
</template>
