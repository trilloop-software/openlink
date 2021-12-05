<template>
  <q-page class="flex full-width column justify-center items-center">
    <!-- TEMPORARY BUTTON TO SHOW HOW TO CALL RUST FUNCTIONS FROM VUE -->
    <q-btn push color="secondary" label="Test" @click="test" />

    <img id="logo" alt="OpenLink logo" src="../assets/logo.svg">

    <template v-if="!connected">
      <connect @connectionSuccess="connected = true" />
    </template>
      
    <template v-else>
      <login />
    </template>

  </q-page>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

/**
 * For some reason, VS Code on John's PC cannot find '@/components/Connect.vue'
 * Shows an error in the IDE
 * But the app still runs fine
 */
import Connect from '@/components/Connect.vue'
import Login from '@/components/Login.vue'

export default {
  name: 'Start',
  components: {
    Connect,
    Login
  },
  setup: () => {
    const connected = ref(false)

    // TEMPORARY FUNCTION TO SHOW HOW TO CALL RUST FUNCTIONS FROM VUE
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
      connected,
      test, // TEMPORARY
    }
  }
}
</script>

<style lang="sass" scoped>
#logo
  width: 600px
</style>
