<template>
  <q-page class="flex full-width column justify-center items-center">
    <!-- TEMPORARY BUTTON TO SHOW HOW TO CALL RUST FUNCTIONS FROM VUE -->
    <q-btn push color="secondary" label="Test" @click="test" />

    <img id="logo" alt="OpenLink logo" src="../assets/logo.svg">

    <!-- JUST FOR SHOW, IMPLEMENT LATER, QUASAR ALSO HAS BUILT IN VALIDATION THAT WE CAN USE WITH :rules PROP -->
    <q-input class="login-inputs" v-model="username" filled :type="text" placeholder="Username" />
    <q-input class="login-inputs" v-model="password" filled :type="hidePassword ? 'password' : 'text'" placeholder="Password">
      <template v-slot:append>
        <q-icon
          :name="hidePassword ? 'visibility_off' : 'visibility'"
          class="cursor-pointer"
          @click="hidePassword = !hidePassword"
        />
      </template>
    </q-input>
    <q-btn class="login-inputs" push color="primary" label="Login" />

  </q-page>
</template>

<style>
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export default {
  name: 'Login',
  setup: () => {
    const hidePassword = ref(true)
    const password = ref("")
    const username = ref("")

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
      hidePassword,
      password,
      test, // TEMPORARY
      username
    }
  }
}
</script>

<style lang="sass" scoped>
#logo
  width: 600px

.login-inputs
  width: 300px
  margin-top: 10px
</style>
