<template>
  <q-input class="connect-inputs" v-model="ipaddr" filled :type="text" placeholder="IP Address" />
  <q-input class="connect-inputs" v-model="port" filled :type="text" placeholder="Port" />
  <q-btn class="connect-inputs" push color="primary" label="Connect" @click="connectToPod" />
</template>

<script lang="ts">
//import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export default {
  name: 'Connect',
  emits: ['connectionSuccess'],
  setup: (props: any, context: any) => {
    const ipaddr = ref("")
    const port = ref("")

    function connectToPod() {
      context.emit('connectionSuccess')
    }

    return {
      connectToPod,
      ipaddr,
      port
    }
  }
}
</script>

<style lang="sass" scoped>
.connect-inputs
  width: 300px
  margin-top: 10px
</style>
