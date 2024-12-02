<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import {readFile, BaseDirectory} from '@tauri-apps/plugin-fs';
import {readImageAsBase64} from "../../services/imgService.ts";

let img3 = `D:/argus/img/局部/5e9324ca411fa3f6.jpg`
const img6 = ref("")

readFile(img3)
    .then((data) => {
      const base64String = btoa(
          new Uint8Array(data).reduce((acc, byte) => acc + String.fromCharCode(byte), "")
      );
      img6.value = `data:image/jpeg;base64,${base64String}`
    })
    .catch((err) => {
      console.error("Error reading file:", err);
    });


function imgAsBase64() {
  let stringPromise = readImageAsBase64("D:\\argus\\img\\局部\\3f160e3827ea5aa149f3301a56b4f0a5.jpg");
  stringPromise.then((data) => {
    img6.value = `data:image/jpeg;base64,${data}`;
  })
}
</script>

<template>
  <button @click="imgAsBase64">img base64</button>
  <h1>图像展示123</h1>
  <hr>
  <img :src="img6" alt="examp123le1"/>
</template>

<style scoped>

</style>
