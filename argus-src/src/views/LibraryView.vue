<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Plus } from '@element-plus/icons-vue'
import type { photoStorageType } from '@/types/photoStorage.type'
import { open } from '@tauri-apps/plugin-dialog'
import { addPhotoStorage, getAllLibrary } from '@/services/libraryService'

// 输入框输入的值
const input = ref('')

// 页面挂在后，查询所有路径
onMounted(() => {
  let basicSetting = getAllLibrary()
  basicSetting.then((res) => {
    let parse: photoStorageType = JSON.parse(res)
    console.log(parse)
  })
})

async function selectFolder() {
  try {
    const selectedFile = await open({
      multiple: false, // 设置是否允许多选
      directory: true // 设置为 true 以选择文件夹
    })

    if (selectedFile) {
      console.log('选中的文件夹:', selectedFile)
      input.value = selectedFile
    } else {
      console.log('用户取消了选择')
    }
  } catch (error) {
    console.error('文件选择失败:', error)
  }
}

/**
 * 添加文件路径
 */
function addFolder() {
  let item: photoStorageType = {
    id: 0,
    img_paths: input.value,
    is_enable: true,
    is_delete: false,
    create_time: 0,
    update_time: 0
  }
  console.log("123123123123");
  let stringPromise = addPhotoStorage(item)
  stringPromise
    .then((value) => {
      console.log(value)
    })
    .catch((err) => {
      console.error(err)
    })
}

const checkList = ref(['Value selected and disabled', 'Value A'])
</script>

<template>
  <div class="flex flex-col w-full">
    <div class="flex flex-col p-3">
      <span class="mb-2">选择要检索的照片路径!</span>
      <!--  图像路径选择-->
      <div class="flex">
        <el-input class="flex-1" size="large" v-model="input" placeholder="输入或选择路径.">
          <template #suffix>
            <el-button @click="selectFolder">选择文件夹</el-button>
          </template>
        </el-input>

        <el-button class="ml-2 w-40" @click="addFolder" type="primary" size="large">
          <el-icon style="vertical-align: middle">
            <Plus />
          </el-icon>
          <span style="vertical-align: middle" > 添加 </span>
        </el-button>
      </div>

      <!--  图像路径展示-->
      <div>
        <el-checkbox-group v-model="checkList">
          <el-checkbox label="Option A" value="Value A" />
          <el-checkbox label="Option B" value="Value B" />
          <el-checkbox label="Option C" value="Value C" />
        </el-checkbox-group>
      </div>

      <!--  图像路径启用-->
      <!--  图像路径获取-->
      <h1>测试</h1>
    </div>
    <el-divider border-style="dotted" />
  </div>
</template>

<style scoped></style>
