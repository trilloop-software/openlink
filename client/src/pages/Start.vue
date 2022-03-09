<template>
  <q-page class="flex full-width column justify-center items-center">

    <img id="logo" alt="OpenLink logo" src="../assets/logo.svg">

    <template v-if="!states.connectState">
      <connect @connection-success="connectionSuccess" @connection-error="connectionError" @warning="warning" />
    </template>
      
    <template v-else>
      <template v-if="!states.loginState">
        <login @login-success="loginSuccess" @login-error="loginError" @set-usergroup="setUsergroup" @warning="warning" />
      </template>
      <template v-else>
        <span class="text-center text-h4 text-primary text-weight-medium">LOGGED IN</span>
      </template>
    </template>

    <notification v-model:show="notifyShow" :kind="notifyKind" :msg="notifyMsg" />
  </q-page>
</template>

<script lang="ts">
import { ref } from 'vue'

import Connect from '@/components/Connect.vue'
import Login from '@/components/Login.vue'
import Notification from '@/components/Notification.vue'
import { statesStore } from '@/stores/states'

export default {
  name: 'Start',
  components: {
    Connect,
    Login,
    Notification,
  },
  setup: () => {
    const states = statesStore()

    const notifyShow = ref(false)
    const notifyKind = ref('positive')
    const notifyMsg = ref('')

    function connectionSuccess(response) {
      states.connectState = true
      notifyShow.value = true
      notifyKind.value = 'positive'
      notifyMsg.value = response
    }

    function connectionError(error) {
      states.connectState = false
      notifyShow.value = true
      notifyKind.value = 'negative'
      notifyMsg.value = error
    }

    function loginSuccess(response) {
      states.loginState = true
      notifyShow.value = true
      notifyKind.value = 'positive'
      notifyMsg.value = response
    }

    function loginError(error) {
      states.loginState = false
      notifyShow.value = true
      notifyKind.value = 'negative'
      notifyMsg.value = error
    }

    function setUsergroup(response) {
      states.usergroupState = parseInt(response)
    }

    function warning(error) {
      notifyShow.value = true
      notifyKind.value = 'warning'
      notifyMsg.value = error
    }

    return {
      notifyShow,
      notifyKind,
      notifyMsg,
      connectionSuccess,
      connectionError,
      loginSuccess,
      loginError,
      setUsergroup,
      states,
      warning,
    }
  }
}
</script>

<style lang="sass" scoped>
#logo
  width: 600px
</style>
