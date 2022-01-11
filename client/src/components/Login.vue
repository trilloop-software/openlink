<template>
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
</template>

<script lang="ts">
//import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export default {
  name: 'Login',
  setup: () => {
    const hidePassword = ref(true)
    const password = ref("")
    const username = ref("")

    return {
      hidePassword,
      password,
      username
    }
  }
}
</script>

<style lang="sass" scoped>
.login-inputs
  width: 300px
  margin-top: 10px
</style>
