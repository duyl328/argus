<script setup lang="ts">

import {open, save, confirm, message} from "@tauri-apps/plugin-dialog";


// 创建 Yes/No 对话框
async function askYesNo() {
  confirm("Do you want to do something?")
      .then((res) => {
        console.log(res);
      })
      .catch((reason) => {
        console.error('报错', reason);
      });
}

</script>

<template>
  <button @click="askYesNo">Yes/No 对话框</button>
</template>

<style scoped>

</style>
