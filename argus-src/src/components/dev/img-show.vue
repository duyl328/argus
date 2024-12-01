<script setup lang="ts">
import {invoke, convertFileSrc} from "@tauri-apps/api/core";
import {getImageAbsolutePathCommand} from "../../command.ts";
import {computed, onMounted} from "vue";
import {getImg} from "../../services/imgService.ts";
import {path} from "@tauri-apps/api";
import {readFile, BaseDirectory} from '@tauri-apps/plugin-fs';
import {checkDirectoryAccess} from "../../services/fileService.ts";

let img = ""
const imgPath = computed<string>(
    {
      get() {
        return img
      }, set(newValue: string) {
        // console.log(newValue,'111111111122')
        img = `tauri://file/${newValue}`
        // console.log(img)
      }
    }
)

function getPath() {
  getImg().then(res => {
    // console.log(res)
    imgPath.value = res
    // console.log('111111111',imgPath.value)
  })
}

// http://localhost:1420/@fs/D:/argus/img/%E5%B1%80%E9%83%A8/5e9324ca411fa3f6.jpg
// http://localhost:1420/@fs/D:/argus/img/%E5%B1%80%E9%83%A8/5e9324ca411fa3f6.jpg

// http://asset.localhost/tauri%3A%2F%2Ffile%2FD%3A%2Fargus%2Fimg%2F%25E5%25B1%2580%25E9%2583%25A8%2F5e9324ca411fa3f6.jpg
// http://asset.localhost/tauri%3A%2F%2Ffile%2FD%3A%2Fargus%2Fimg%2F%E5%B1%80%E9%83%A8%2F5e9324ca411fa3f6.jpg
onMounted(() => {
  getPath()
})


let img1 = `tauri://file/D:/argus/img/%E5%B1%80%E9%83%A8/5e9324ca411fa3f6.jpg`
let img3 = `D:/argus/img/局部/5e9324ca411fa3f6.jpg`
let img4 = decodeURIComponent(img1);
console.log('pppppppppppppp', img4)
let s = convertFileSrc(img4);
console.log(s)
let img2 = s
let img5 = `http://localhost:1420/@fs/${img3}`;


// const updatePath = () => {
//   setImagePath(`tauri://file/path/to/image.png?time=${Date.now()}`);
// };


const home = path.homeDir();
home.then((res) => {
  console.log(res)
})


// const icon = readFile('5e9324ca411fa3f6.jpg', {
//   baseDir: BaseDirectory.Desktop,
// });
// icon.then((res) => {
//   console.log('11111111', res)
// })
//
//
// readFile(img3)
//     .then((data) => {
//       console.log("File content:", data);
//     })
//     .catch((err) => {
//       console.error("Error reading file:", err);
//     });


checkDirectoryAccess("C:/").then((res) => {
  console.log(res)
})
checkDirectoryAccess("D:/").then((res) => {
  console.log(res)
})
checkDirectoryAccess("E:/").then((res) => {
  console.log(res)
})
checkDirectoryAccess("D:/argus").then((res) => {
  console.log(res)
})
checkDirectoryAccess("C:\\Windows\\System32").then((res) => {
  console.log(res)
})


</script>

<template>
  <h1>图像展示123</h1>
  <hr>
  <img :src="img1" alt="exam123ple"/>
  <img :src="img2" alt="example"/>
  <img :src="img5" alt="example1"/>
  <!--    <img src="tauri://file/D:/argus/img/局部/5e9324ca411fa3f6.jpg" alt="example" />-->
  <!--  <img :src="imgPath" alt="example"/>-->

</template>

<style scoped>

</style>
<!--D:\argus\img\局部\5e9324ca411fa3f6.jpg-->
<!--  D:\pt\[Onlyfans]Littlesulaa 韓國推特絕美反差女神合集[132P50V10.2G]\P\47.jpg-->
<!-- 软件目前无法访问E盘文件夹 -->
<!-- 使用tauri://file包裹路径的优势？使用tauri://file时路径更改页面图像无法再更新 -->
