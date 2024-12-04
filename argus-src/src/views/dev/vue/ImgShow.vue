<script setup lang="ts">

import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { readImageAsBase64 } from '@/services/imgService'

let imgBase64 = ref("")

async function selectPhoto() {
  try {
    const selectedFile = await open({
      multiple: false, // 设置是否允许多选
      filters: [
        {
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'gif'], // 限定图片文件类型
        },
      ],
    });

    if (selectedFile) {
      console.log('选中的文件:', selectedFile);
      // 在这里处理选中的照片文件路径
      let stringPromise = readImageAsBase64(selectedFile)
      stringPromise.then((value) => {
        imgBase64.value = `data:image/jpeg;base64,${value}`;
      })

    } else {
      console.log('用户取消了选择');
    }
  } catch (error) {
    console.error('文件选择失败:', error);
  }
}

</script>

<template>
  <h1>图像展示</h1>
  <el-button @click="selectPhoto">选择文件</el-button>

  <img :src="imgBase64" alt="">
</template>

<style scoped>

</style>
