<template>
  <q-input class="connect-inputs" v-model="ipaddr" filled :type="text" placeholder="IP Address" />
  <q-input class="connect-inputs" v-model.number="port" filled :type="text" placeholder="Port" />
  <q-btn class="connect-inputs" push color="primary" label="Connect" @click="connectToPod" />
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export default {
  name: 'Connect',
  emits: ['connectionSuccess','connectionError','warning'],
  setup: (props: any, { emit }) => {
    const ipaddr = ref("")
    const port = ref("")

    function connectToPod() {
      if (!checkIp()) {
        emit('warning', 'Please enter a valid IP')
        return
      }

      if (!checkPort()) {
        emit('warning', 'Please enter a valid port')
        return
      }

      invoke("connect", { addr: `${ipaddr.value}:${port.value}` })
        .then((response) => {
          emit('connectionSuccess', response)
        })
        .catch((error) => {
          emit('connectionError', error)
        })
    }

    function checkIp() {
      const numArray = ipaddr.value.split('.').map(Number)

      if (numArray.length != 4) {
        return false
      } else {
        if (numArray.every(x => isNaN(x))) {
          return false
        } else {
          if (numArray.every(x => x >= 0 && x <= 255)) {
            return true
          } else {
            return false
          }
        }
      }
    }

    function checkPort() {
      if (port.value == "") {
        return false
      }
      
      const num = Number(port.value)

      if (isNaN(num)) {
        return false
      } else {
        if (num > 65535 || num < 0) {
          return false
        } else {
          return true
        }
      }
    }

    return {
      connectToPod,
      ipaddr,
      port,
    }
  }
}
</script>

<style lang="sass" scoped>
.connect-inputs
  width: 300px
  margin-top: 10px
</style>
