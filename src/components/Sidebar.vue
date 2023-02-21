<script setup lang="ts">
import { Close } from "@element-plus/icons-vue";
import { ElMessage } from "element-plus";
import { inject, Ref } from "vue";

const tags = inject('tags') as Ref<string[]>;
const refreshData = inject('refreshData') as () => void;

const current_tag = inject('current_tag') as Ref<string>;
const special_tag = inject('special_tag') as Ref<string>;
const dialogFormVisible = inject('dialogFormVisible') as Ref<boolean>;

const getAllTasks = () => {
  current_tag.value = '';
  special_tag.value = 'all';
  refreshData();
}

const getTodayTasks = () => {
  current_tag.value = '';
  special_tag.value = 'today';
  refreshData();
}

const getTomorrowTasks = () => {
  current_tag.value = '';
  special_tag.value = 'tomorrow';
  refreshData();
}

const getTagTasks = (tag: string) => {
  current_tag.value = tag;
  special_tag.value = '';
  refreshData();
}

const toDo = () => {
  ElMessage({
    message: 'Unimplemented.',
    type: 'warning',
    duration: 1000
  })
}
</script>

<template>
  <div class="sidebar_block">
    <div class="sidebar_block_inner"
      style="text-align: center; padding-top: 4px; padding-bottom: 4px; border: 2px dashed; border-color: #3375b9; border-radius: 20px;">
      <el-button circle type="info" @click="dialogFormVisible = true">
        <el-icon>
          <Plus />
        </el-icon>
      </el-button>

      <el-button circle type="success" @click="toDo">
        <el-icon>
          <Check />
        </el-icon>
      </el-button>

      <el-button circle type="danger" @click="toDo">
        <el-icon>
          <Close />
        </el-icon>
      </el-button>
    </div>
  </div>

  <div class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#3375b9" @click="getAllTasks">
        <el-icon>
          <More />
        </el-icon> All Tasks
      </el-button>
    </div>
  </div>

  <div class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#3375b9" @click="getTodayTasks">
        <el-icon>
          <Flag />
        </el-icon> Today
      </el-button>
    </div>
  </div>

  <div class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#3375b9" @click="getTomorrowTasks">
        <el-icon>
          <Sunny />
        </el-icon> Tomorrow
      </el-button>
    </div>
  </div>

  <div v-for="tag in tags" class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#4e8e2f" @click="getTagTasks(tag)">
        <el-icon>
          <ArrowRight />
        </el-icon>{{ tag }}
      </el-button>
    </div>
  </div>
</template>

<style>
.sidebar_block {
  min-height: 45px;
  max-height: 100px;
  min-width: 255px;
  width: 100%;
  position: relative;
  color:  #a6a9ad;
}

.sidebar_block_inner {
  position: absolute;
  width: 90%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>