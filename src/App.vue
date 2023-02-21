<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
import Todolist from "./components/Todolist.vue";
import { provide, ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri';

interface Task {
  id: number,
  detail: string,
  year: number,
  month: number,
  day: number,
  tag: string
};

const current_tag = ref<string>('');
provide('current_tag', current_tag);

const special_tag = ref<string>('');
provide('special_tag', special_tag);

const dialogFormVisible = ref(false);
provide('dialogFormVisible', dialogFormVisible);

const tags = ref<string[]>([]);
provide('tags', tags);
invoke('get_tags').then((res: any) => tags.value = res);

const tasks = ref<Task[]>([]);
provide('tasks', tasks);
invoke('get_tasks').then((res: any) => tasks.value = res);

const refreshData = () => {
  console.log('refreshData', current_tag.value, special_tag.value);
  invoke('get_tags').then((res: any) => tags.value = res);

  if (special_tag.value == '') {
    invoke('get_tasks_by_tag', { tag: current_tag.value }).then((res: any) => tasks.value = res);
  } else if (special_tag.value == 'all') {
    invoke('get_tasks').then((res: any) => tasks.value = res);
  } else {
    const date = new Date();
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    const day = date.getDate();
    if (special_tag.value == 'today') {
      invoke('get_tasks_by_days', {
        year,
        month,
        day,
        beginOffset: 0,
        endOffset: 0
      }).then((res: any) => tasks.value = res);
    } else if (special_tag.value == 'tomorrow') {
      invoke('get_tasks_by_days', {
        year,
        month,
        day,
        beginOffset: 1,
        endOffset: 1
      }).then((res: any) => tasks.value = res);
    }
  }
}

provide('refreshData', refreshData);
</script>

<template>
  <div class="background">
    <el-row :gutter="10">
      <el-col :span="6" :offset="0" style="max-width: 400px;">
        <Sidebar />
      </el-col>
      <el-col :span="17" :offset="0">
        <Todolist />
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
