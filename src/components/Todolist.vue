<script setup lang="ts">
import { Check } from "@element-plus/icons-vue";
import { inject, provide, reactive, Ref, ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri';
import { ElMessage, FormInstance, FormRules } from "element-plus";

interface Task {
  id: number,
  detail: string,
  year: number,
  month: number,
  day: number,
  tag: string
};

const tasks = inject('tasks') as Task[];
const refreshData = inject('refreshData') as () => void;
const dialogFormVisible = inject('dialogFormVisible') as Ref<boolean>;

const taskForm = reactive({
  detail: '',
  tag: '',
  date: new Date()
});

const formRules = reactive<FormRules>({
  detail: [
    { required: true, message: 'Please input task detail', trigger: 'blur' },
    { min: 1, max: 80, message: 'Length should between 1 and 80', trigger: 'blur' },
  ],
  tag: [
    { required: true, message: 'Please input task tag', trigger: 'blur' },
    { min: 1, max: 15, message: 'Length should between 1 and 15', trigger: 'blur' },
  ],
  date: [
    { required: true, message: 'Please pick a date', trigger: 'blur' },
  ],
})

const addTask = () => {
  console.log('addTask', taskForm.detail, taskForm.tag, taskForm.date);
  if (taskForm.detail == '' || taskForm.tag == '' || taskForm.tag == '') {
    showError('Please complete the form');
  } else {
    // add_task(detail: String, year: i32, month: i32, day: i32, tag: String) {
    invoke('add_task', {
      detail: taskForm.detail,
      year: taskForm.date.getFullYear(),
      month: taskForm.date.getMonth() + 1,
      day: taskForm.date.getDate() + 1,
      tag: taskForm.tag
    });
    taskForm.detail = '';
    taskForm.tag = '';
    taskForm.date = new Date();
    refreshData();
  }
}

const showError = (message: string) => {
  ElMessage({
    message,
    type: 'error',
    duration: 1500
  })
};

const deleteTask = (id: number) => {
  console.log('delete', id);
  invoke('delete_task', { id });
  console.log('delete_1');
  refreshData();
  console.log('delete_2');
};

</script>

<template>
  <div style="color: #3375b9; font-size: 150%;">
    🍩Todolist
  </div>
  <div v-for="task in tasks" class="task_block">
    <div class="task_block_inner">
      <el-button-group style="width: 100%; display: flex;">
        <el-button style="width: 45px;" color="#66b1ff" @click="deleteTask(task.id)">
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
        <el-button color="#2a598a" style="width: 120px;">
          <!-- style="width: 150px; text-align: center;" -->
          {{ task.year }} / {{ task.month }} / {{ task.day }}
        </el-button>
      </el-button-group>
    </div>
  </div>

  <el-dialog v-model="dialogFormVisible" title="Add a task" class="form_dialog">
    <!-- style="background-color: rgb(45, 46, 47); color: azure;" -->
    <el-form :model="taskForm" :rules="formRules" label-width="100px" label-position="right">
      <el-form-item label="Task Detail" prop="detail">
        <el-input v-model="taskForm.detail" maxlength="80" autocomplete="off" />
      </el-form-item>
      <el-form-item label="Tag" prop="tag">
        <el-input v-model="taskForm.tag" maxlength="15" autocomplete="off" />
      </el-form-item>
      <el-form-item label="Date" prop="date">
        <el-date-picker v-model="taskForm.date" type="date" placeholder="Pick a date" style="width: 100%" />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button type="info" @click="dialogFormVisible = false">Cancel</el-button>
        <el-button type="primary" @click="addTask">
          Confirm
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<style>
.task_block {
  min-height: 50px;
  max-height: 100px;
  width: 100%;
  position: relative;
  color:  #a6a9ad;
}

.task_block_inner {
  position: absolute;
  width: 100%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.form_dialog .el-dialog__header {
  background-color: rgb(45, 46, 47);
  margin: 0;
}

.form_dialog .el-dialog__title {
  color: #a6a9ad;
  background-color: rgb(45, 46, 47);
}

.form_dialog .el-dialog__body {
  background-color: rgb(45, 46, 47);
}

.form_dialog .el-form-item__label {
  color: #a6a9ad;
}

.form_dialog .el-input__wrapper {
  color: #a6a9ad;
  background-color: rgb(45, 46, 47);
}

.form_dialog .el-input__inner {
  color: azure;
}

.form_dialog .el-dialog__footer {
  background-color: rgb(45, 46, 47);
}
</style>