<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
import Todolist from "./components/Todolist.vue";
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri';

interface Task {
  id: number,
  detail: string,
  year: number,
  month: number,
  day: number,
  tag: string
};

let tags = ref<string[]>([]);
invoke('get_tags').then((res: any) => tags.value = res);

let tasks = ref<Task[]>([]);
invoke('get_tasks').then((res: any) => tasks.value = res);

console.log(tags.value);
console.log(tasks.value);

</script>

<template>
  <div class="background">
    <el-row :gutter="10">
      <el-col :span="6" :offset="0" style="max-width: 400px;">
        <Sidebar :tags="tags"/>
      </el-col>
      <el-col :span="17" :offset="0">
        <Todolist :tasks="tasks" />
      </el-col>
    </el-row>
  </div>
</template>

<style>
#app {
  margin: 0%;
  padding: 0%;
  border: 0px;
  position: fixed;
  top: 0px;
  left: 0px;
  height: 100%;
  width: 100%;
  background-color: rgb(45, 46, 47);
}
</style>
