<script setup lang="ts">
import { Check } from "@element-plus/icons-vue";
import { ref, toRefs } from "vue";

interface Task {
  id: number,
  detail: string,
  year: number,
  month: number,
  day: number,
  tag: string
};

const props = defineProps({
  tasks: Array<Task>
})

let { tasks } = toRefs(props);
</script>

<template>
  <div style="color: #3375b9; font-size: 150%;">
    <el-icon>
      <Menu />
    </el-icon>
    待办事项
  </div>
  <div v-for="task in tasks" class="task_block">
    <div class="task_block_inner">
      <el-button-group style="width: 100%; display: flex;">
        <el-button style="width: 50px;" color="#66b1ff">
          <el-icon>
            <Check />
          </el-icon>
        </el-button>
        <el-button style="flex: 1;" color="#3375b9">
          <div style="position: absolute; left: 10px;">
            {{ task.detail }}
          </div>
        </el-button>
        <el-button color="#4e8e2f">
          {{ task.tag }}
        </el-button>
        <el-button color="#2a598a">
          <!-- style="width: 150px; text-align: center;" -->
          {{ task.year }} / {{ task.month }} / {{ task.day }}
        </el-button>
      </el-button-group>
    </div>
  </div>
</template>

<style>
.task_block {
  min-height: 50px;
  max-height: 100px;
  width: 100%;
  position: relative;
  color: azure;
}

.task_block_inner {
  position: absolute;
  width: 100%;
  /* max-width: 300px; */
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>