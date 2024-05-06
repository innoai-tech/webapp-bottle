<script setup lang="ts">
import {  onMounted, ref } from 'vue';
import { Input, Modal } from 'ant-design-vue';
import { invoke } from "@tauri-apps/api/tauri";

const modalVisible = ref(false)
const url = ref(localStorage.getItem('url') || 'http://172.20.40.71:20080')


const onOpenSettings = () => {
  modalVisible.value=true
}

const onBlur = (e:any) => {
  const _url = e.target.value
  url.value = _url
  localStorage.setItem('url', url.value)
}

onMounted(async () => {

  // 获取所有可用显示器
invoke("get_displays").then(res => {

  console.log(111,res)
})
})
</script>

<template>
  <div class="container">
    <Modal :open="modalVisible" title="系统配置" @cancel="modalVisible=false" @ok="modalVisible=false">
      <Input :key="url" :default-value="url" placeholder="输入访问地址" @blur="onBlur"></Input>
    </Modal>

    <!-- <img class="loading" src="./loading.gif" alt=""></div> -->
    <iframe :key="url" :src="url" allowfullscreen allowpaymentrequest allowtransparency title="W3Schools Free Online Web Tutorials"></iframe>


    <div class="btn" @click="onOpenSettings"></div>
  </div>
</template>

<style scoped>
.container, .container iframe {
  width:100vw;
  height: 100vh;
  position: relative;
  position: 1;
}
.loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate3d(-50%,-50%,0);
}
.btn {
  position: absolute;
  right: 0;
  bottom:0;
  z-index: 2;
  width: 20px;
  height: 20px;
}
</style>
