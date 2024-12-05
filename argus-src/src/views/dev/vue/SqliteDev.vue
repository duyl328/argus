<script setup lang="ts">
import Database from '@tauri-apps/plugin-sql'
import { getAllPost, insertPost } from '@/services/storageService'

async function getAllPost1() {
  // 初始化数据库连接
//   const db = await Database.load('sqlite://./sqlite.db')
//
// // 创建表
//   await db.execute(
//     `CREATE TABLE IF NOT EXISTS user (
//     id INTEGER PRIMARY KEY AUTOINCREMENT,
//     name TEXT NOT NULL,
//     age INTEGER
//   )`
//   );
//
// // 插入数据
//   await db.execute("INSERT INTO user (name, age) VALUES (?, ?)", ["Alice", 25]);
//   await db.execute("INSERT INTO user (name, age) VALUES (?, ?)", ["Bob", 30]);
//
// // 查询数据
//   const rows = await db.select("SELECT * FROM user");
//   console.log(rows);
   getAllPost()
}

function insertPostBase() {
  insertPost("标题1","内容1")
}
</script>

<template>
  <h1>数据库</h1>
  <br>
  <el-button @click="getAllPost1">获取所有数据</el-button>
  <el-button @click="insertPostBase">插入数据</el-button>
</template>

<style scoped></style>
