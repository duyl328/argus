<script setup lang="ts">
import { onMounted, type Reactive, type Ref, reactive, ref } from 'vue'
import { Delete, Plus } from '@element-plus/icons-vue'
import type { photoStorageType } from '@/types/photoStorage.type'
import { open } from '@tauri-apps/plugin-dialog'
import {
  addPhotoStorage,
  deletePhotoStorage,
  getAllLibrary,
  updatePhotoStorage
} from '@/services/libraryService'
import { updatePhotoStorageCommand } from '@/constants/command'

// 输入框输入的值
const input = ref('')
// 所有的路径展示
let folders: Reactive<photoStorageType[]> = reactive([])
const checkList = ref([])

// 页面挂在后，查询所有路径
onMounted(() => {
  getAllData()
})

/**
 * 获取所有路径数据
 */
function getAllData() {
  let basicSetting = getAllLibrary()
  basicSetting.then((res) => {
    let parse: photoStorageType[] = JSON.parse(res)
    folders.length = 0
    folders.push(...parse)
  })
}

/**
 * 选择文件夹
 */
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
  let stringPromise = addPhotoStorage(input.value)
  stringPromise
    .then((value) => {
      console.log(value)
      // 更新路径
      getAllData()
      // 清空添加
      input.value = ''
    })
    .catch((err) => {
      console.error(err)
    })
}

/**
 * 删除指定路径
 */
function deletePhotoPath(id: number) {
  deletePhotoStorage(id)
    .then(() => {
      console.log('删除成功')
      const index = folders.findIndex((folder) => folder.id === id)
      if (index !== -1) {
        folders.splice(index, 1)
      }
      console.log(folders)
    })
    .catch((err) => {
      console.error('删除失败:', err)
    })
}

/**
 * 更新选中状态
 */
function pathSelectChange(items: photoStorageType) {
  items.is_enable = !items.is_enable
  updatePhotoStorage(items)
    .then((res) => {
      console.log('更新成功')
      const index = folders.findIndex((folder) => folder.id === items.id)
      if (index !== -1) {
        folders[index] = { ...folders[index], is_enable: !folders[index].is_enable }
      }
    })
    .catch((err) => {
      console.error(err)
    })
}
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
          <span style="vertical-align: middle"> 添加 </span>
        </el-button>
      </div>

      <!--  图像路径展示-->
      <div>
        <!--  图像路径启用-->
        <el-checkbox
          @change="pathSelectChange(item)"
          class="mt-4"
          size="large"
          border
          v-for="item in folders"
          :key="item.id"
          :label="item.img_paths"
          :value="item.img_paths"
          :checked="item.is_enable"
        >
          <template #default>
            <span>
              {{ item.img_paths }}
            </span>
            <el-popconfirm @confirm="deletePhotoPath(item.id)" title="Are you sure to delete this?">
              <template #reference>
                <el-button size="small" class="ml-5 mr-1" type="info" :icon="Delete" circle />
              </template>
            </el-popconfirm>
          </template>
        </el-checkbox>
      </div>
    </div>
    <el-divider border-style="dotted" />

    <!--    该数据需要存储到数据库-->

    <!--    重新检索【默认增量检索】-->
    <div class="flex lib-option w-64">
        <span class="mt-1 iconfont icon-zhongxin search-again"></span>
        <div class="flex flex-col">
          <el-checkbox class="text-4xl" label="重新检索" size="large" />
          <span class="text-wrap">放弃所有已索引文件，全部重新检索，默认增量检索</span>
      </div>
    </div>

    <!--    清理-->
  </div>
</template>

<style  lang="scss">
.lib-option {
  /* 重新检索的 icon */
  .search-again{
    margin-right: .7rem;
    font-size: 1.2rem;
  }

  /* 放大复选框组件 */
  .el-checkbox.el-checkbox--large {
    /* 选择框 */
    .el-checkbox__inner {
      &::after{
        @apply h-3 w-2;
      }
      @apply h-5 w-5;
    }
    /* 展示文字*/
    .el-checkbox__label{
      font-size: 1.5em;
    }
  }
}



</style>
