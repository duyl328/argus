<script setup lang="ts">
import { onMounted, onUnmounted, reactive, type Reactive, type Ref, ref, watch, type WatchStopHandle } from 'vue'
import { Delete, Plus } from '@element-plus/icons-vue'
import type { photoStorageType } from '@/types/photoStorage.type'
import { open } from '@tauri-apps/plugin-dialog'
import { addPhotoStorage, deletePhotoStorage, getAllLibrary, updatePhotoStorage } from '@/services/libraryService'
import { addPhotoRetrieveTask } from '@/services/globalService'
import { addListener, removeListener } from '@/services/emits/base'
import emitOrder from '@/constants/emitOrder'
import { getAppStatus } from '@/AppStatus'
import type { Event } from '@tauri-apps/api/event'
import type { loadMsg } from '@/models/globalErrorMsg'

// 输入框输入的值
const input = ref('')
// 所有的路径展示
let folders: Reactive<photoStorageType[]> = reactive([])

/**
 * 正在处理任务
 */
const taskName = ref('')

// region 数据库操作
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

// endregion

// region 进度条
const basicProcessColor: string = ''
const activeProcessColor: string = '#dcdfe6'

/**
 * 已完成任务数
 */
let doneTaskNum = 0

/**
 * 是否正在加载
 */
const isLoading = ref(false)

/**
 * 监视任务属性
 */
watch(isLoading, (value, oldValue, onCleanup) => {
  if (value) {
    processColor.value = basicProcessColor
    processProgress.value = 0
    processStriped.value = false
    processStriped.value = true
    processStrokeWidth.value = 18
  } else {
    processColor.value = activeProcessColor
    processProgress.value = 100
    processStrokeWidth.value = 1
    processStriped.value = false
    taskName.value = ''
  }
})

/**
 * 进度条进度
 */
const processProgress = ref(100)

/**
 * 进度条颜色
 */
const processColor = ref<string>('')
processColor.value = activeProcessColor

/**
 * 进度条高度
 */
const processStrokeWidth = ref(1)

/**
 * 进度条条纹
 */
const processStriped = ref(false)

/**
 * 进度条展示文字
 */
let processFormat = (percentage: number): string => {
  if (processStriped.value) {
    const factor = Math.pow(10, 2)
    return Math.round(percentage * factor) / factor + '%'
  } else {
    return ''
  }
}

/**
 * 错误信息展示
 */
const errMsg: Ref<string[]> = ref([])

onMounted(() => {
  // 绑定事件
})

// endregion

// region 开始 和 取消

/**
 * 是否重新检索
 */
const isReRetrieve = ref(false)

/**
 * 开始
 */
function retrieveStart() {
  const newArray: string[] = folders.map((folder) => folder.img_paths as string)
  let promise = addPhotoRetrieveTask(newArray)
  if (isLoading.value) {
    return
  }
  // 进度条状态调整
  isLoading.value = true
  processProgress.value = 0
  doneTaskNum = 0
}

/**
 * 取消
 */
function retrieveCancel() {
  console.log('取消')
}

/**
 * 进度及现在正在加载的信息展示
 */
let errorListener = (event: unknown) => {
  let event1 = event as Event<string>
  errMsg.value.push(event1.payload)
  console.log('报错', event1.payload)
}
/**
 * 报错提示
 */
let msgListener = (event: unknown) => {
  let event1 = event as Event<string>
  let msg: loadMsg = JSON.parse(event1.payload)
  processProgress.value = (msg.currentTask / msg.allTask) * 100
  if (processProgress.value === 100) {
    taskName.value = '任务完成!'
  } else {
    taskName.value = msg.taskMsg
  }
}

// endregion

// 获取状态实例
let appStatus = getAppStatus()

// 页面挂在后
onMounted(() => {
  // 查询所有路径
  getAllData()

  // 绑定后台事件
  let stop: WatchStopHandle | null = null
  stop = watch(
    appStatus.emitIsInit,
    (value, onCleanup) => {
      if (value) {
        addListener(emitOrder.photoLoadingMsgTip, msgListener)
        addListener(emitOrder.photoLoadingErrTip, errorListener)
        if (stop != null) {
          stop()
        }
      }
    },
    {
      immediate: true
    }
  )
})

// 页面卸载
onUnmounted(() => {
  removeListener(emitOrder.photoLoadingMsgTip, msgListener)
  removeListener(emitOrder.photoLoadingErrTip, errorListener)
})
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

    <!--    进度条-->
    <div class="mt-5 mb-5">
      <el-progress
        :stroke-width="processStrokeWidth"
        :percentage="processProgress"
        :text-inside="true"
        :color="processColor"
        :format="processFormat"
      />
      <!--      信息提示文字-->
      <div v-show="taskName" class="text-nowrap flex justify-center items-center w-full mt-2">
        <span class="text-nowrap">正在处理任务：</span>
        <span class="text-nowrap">{{ taskName }}</span>
      </div>
    </div>

    <!--    其他选项-->
    <div class="flex flex-row flex-wrap">
      <!--    该数据需要存储到数据库-->

      <!--    重新检索【默认增量检索】-->
      <div class="flex lib-option w-64">
        <span class="mt-1.5 iconfont icon-zhongxin search-again"></span>
        <div class="flex flex-col">
          <el-checkbox class="text-4xl" v-model="isReRetrieve" label="重新检索" size="large" />
          <span class="text-wrap">放弃所有已索引文件，全部重新检索，默认增量检索</span>
        </div>
      </div>

      <!--    清理【清理缩略图】-->
    </div>

    <!--    开始检索与选项-->
    <div class="mt-10 ml-10">
      <!--      取消-->
      <el-button class="w-20" @click="retrieveCancel">取消</el-button>
      <!--      开始构建-->
      <el-button type="primary" @click="retrieveStart" class="ml-10 w-28">
        开始
        <el-icon class="el-icon--right"><span class="iconfont icon-zhongxinkaishi"></span></el-icon>
      </el-button>
    </div>

    <hr class="mt-10 mb-10" v-if="!!errMsg">

    <!--    报错信息展示-->
    <div class="w-80" v-if="!!errMsg">
      <div v-for="msg in errMsg">
        <span>
          {{ msg }}
        </span>
        <hr>
      </div>
    </div>

  </div>
</template>

<style lang="scss">
.lib-option {
  /* 重新检索的 icon */
  .search-again {
    margin-right: 0.7rem;
    font-size: 1.2rem;
  }

  /* 放大复选框组件 */
  .el-checkbox.el-checkbox--large {
    /* 选择框 */
    .el-checkbox__inner {
      &::after {
        @apply h-3.5 w-1.5;
        translate: 2px -1px;
      }

      @apply h-5 w-5;
    }

    /* 展示文字*/
    .el-checkbox__label {
      font-size: 1.5em;
    }
  }
}
</style>
