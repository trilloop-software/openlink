<template>
  <q-page class="flex flex-center">
    <img alt="OpenLink logo" src="../assets/logo.svg" style="width: 400px">
    <q-btn push color="primary" label="Test" @click="test" />
  </q-page>
</template>

<style>
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

export default {
  name: 'Login',
  setup: () => {
    function test() {
      invoke("test")
        .then((response) => {
          alert('Successful: ' + response)
        })
        .catch((error) => {
          alert('Error:' + error)
        })
    }

    return {
      test
    }
  }
}
</script>
