<script setup lang="ts">
import { Close } from "@element-plus/icons-vue";
import { toRefs } from "vue";

const props = defineProps({
  tags: Array<String>
})

let {tags} = toRefs(props);
</script>

<template>
  <div class="sidebar_block">
    <div class="sidebar_block_inner" style="
      text-align: center;
      padding-top: 4px;
      padding-bottom: 4px;
      border: 2px dashed;
      border-color: #3375b9;
      border-radius: 20px;
    ">
      <el-button circle type="info">
        <el-icon>
          <Plus />
        </el-icon>
      </el-button>

      <el-button circle type="success">
        <el-icon>
          <Check />
        </el-icon>
      </el-button>

      <el-button circle type="danger">
        <el-icon>
          <Close />
        </el-icon>
      </el-button>
    </div>
  </div>

  <div class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#3375b9">
        <el-icon>
          <More />
        </el-icon> 全部
      </el-button>
    </div>
  </div>

  <div class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#3375b9">
        <el-icon>
          <Flag />
        </el-icon> 今天
      </el-button>
    </div>
  </div>

  <div class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#3375b9">
        <el-icon>
          <Sunny />
        </el-icon> 明天
      </el-button>
    </div>
  </div>

  <div v-for="tag in tags" class="sidebar_block">
    <div class="sidebar_block_inner">
      <el-button style="width: 100%;" color="#4e8e2f">
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
  min-width: 140px;
  width: 100%;
  position: relative;
  color: azure;
  /* overflow: auto;
  zoom: 1; */
}

.sidebar_block_inner {
  position: absolute;
  width: 90%;
  /* max-width: 300px; */
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>