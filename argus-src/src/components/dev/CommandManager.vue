<script setup lang="ts">
import { onMounted, type Ref, ref } from 'vue'
import { baseInvoke } from '@/utils/commandUtil.js'
import type { CommandType } from '@/types/command.type'
import { logN } from '@/utils/logHelper/logUtils'

// 传递参数
let props = defineProps({
  pro: Array<CommandType>
})
// 使用 ref 使数据变成响应式数据
const modules: Ref<CommandType[]> = ref([...(props.pro || [])])

/**
 * 指令触发
 * @param module
 */
const invokeCommand = async (module: CommandType) => {
  const params = Object.fromEntries(module.params.map((param) => [param.name, param.value]))
  logN.success('前端发送的参数', module.name, params)

  try {
    let s = baseInvoke(module.name, params)
    s.then((res) => {
      logN.warning('后端返回的参数', module.name, res)
      module.result = JSON.stringify(res, null, 2)
    }).catch((err) => {
      module.result = `Error: ${err.message}`
    })
  } catch (error) {
    module.result = `Error: ${error}`
  }
}
</script>

<template>
  <div class="p-6 font-sans">
    <h1 class="text-2xl font-bold text-center mb-8">后端命令测试管理页面</h1>

    <div class="flex flex-wrap gap-6 w-full justify-center">
      <div v-for="(module, index) in modules" :key="index" class="w-full p-4 shadow-md rounded-md">
        <h2 class="text-lg font-semibold mb-2">{{ module.name }}</h2>
        <p class="text-gray-600 mb-4">{{ module.description }}</p>

        <div class="space-y-4 w-full flex flex-row">
          <div
            v-for="(param, paramIndex) in module.params"
            :key="paramIndex"
            class="space-y-1 w-full"
          >
            <label :for="`${module.name}-${param.name}`" class="block font-bold text-red-400">
              {{ param.label }}
            </label>
            <input
              v-if="param.type === 'text'"
              :id="`${module.name}-${param.name}`"
              v-model="param.value"
              :placeholder="param.placeholder"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-md"
            />
            <input
              v-if="param.type === 'checkbox'"
              :id="`${module.name}-${param.name}`"
              v-model="param.value"
              type="checkbox"
              class="rounded"
            />
            <!--            传递到后端的数据基本都是字符、数字、布尔，所以几乎不需要 Select -->
            <!--            <select-->
            <!--              v-if="param.type === 'select'"-->
            <!--              :id="`${module.name}-${param.name}`"-->
            <!--              v-model="param.value"-->
            <!--              class="w-full px-3 py-2 border border-gray-300 rounded-md"-->
            <!--            >-->
            <!--              <option v-for="option in param.options" :value="option" :key="option">-->
            <!--                {{ option }}-->
            <!--              </option>-->
            <!--            </select>-->
          </div>
        </div>

        <button
          @click="invokeCommand(module)"
          class="mt-4 w-full py-2 bg-blue-700 text-white rounded-md hover:bg-blue-600"
        >
          触发 {{ module.name }}
        </button>

        <div v-if="module.result" class="mt-4 p-3 bg-gray-100 border rounded-md">
          <h3 class="font-semibold mb-2">结果</h3>
          <pre class="text-sm whitespace-pre-wrap">{{ module.result }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<style></style>
