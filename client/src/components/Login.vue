<template>
  <q-input class="login-inputs" v-model="username" filled type="text" placeholder="Username" />
  <q-input class="login-inputs" v-model="password" filled :type="hidePassword ? 'password' : 'text'" placeholder="Password">
    <template v-slot:append>
      <q-icon
        :name="hidePassword ? 'visibility_off' : 'visibility'"
        class="cursor-pointer"
        @click="hidePassword = !hidePassword"
      />
    </template>
  </q-input>
  <q-btn class="login-inputs" push color="primary" label="Login" @click="login" />
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

export default {
  name: 'Login',
  emits: ['login-success','login-error','set-usergroup','warning'],
  setup: (props: any, { emit }) => {
    const hidePassword = ref(true)
    const password = ref("")
    const username = ref("")

    const invalid = /[ `!@#$%^&*()_+\-=[\]{};':"\\|,.<>/?~]/

    function login() {
      if (invalid.test(username.value)) {
        emit('warning', 'Invalid characters detected in username')
        return
      }

      if (invalid.test(password.value)) {
        emit('warning', 'Invalid characters detected in password')
        return
      }

      invoke("login", { username: username.value, password: password.value })
        .then((response) => {
          emit('login-success', response[0])
          emit('set-usergroup', response[1])
        })
        .catch((error) => {
          emit('login-error', error)
        })
    }

    return {
      hidePassword,
      login,
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
