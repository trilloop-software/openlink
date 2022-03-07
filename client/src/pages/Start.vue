<template>
  <q-page class="flex full-width column justify-center items-center">

    <img id="logo" alt="OpenLink logo" src="../assets/logo.svg">

    <template v-if="!connected">
      <connect @connectionSuccess="connectionSuccess" @connectionError="connectionError" @warning="warning" />
    </template>
      
    <template v-else>
      <login @loginSuccess="loginSuccess" @loginError="loginError" @warning="warning" />
    </template>

    <notification v-model:show="notifyShow" :kind="notifyKind" :msg="notifyMsg" />
  </q-page>
</template>

<script lang="ts">
import { ref } from 'vue'

import Connect from '@/components/Connect.vue'
import Login from '@/components/Login.vue'
import Notification from '@/components/Notification.vue'

export default {
  name: 'Start',
  components: {
    Connect,
    Login,
    Notification,
  },
  setup: () => {
    const authenticated = ref(false)
    const connected = ref(false)

    const notifyShow = ref(false)
    const notifyKind = ref('positive')
    const notifyMsg = ref('')

    function connectionSuccess(response) {
      connected.value = true
      notifyShow.value = true
      notifyKind.value = 'positive'
      notifyMsg.value = response
    }

    function connectionError(error) {
      notifyShow.value = true
      notifyKind.value = 'negative'
      notifyMsg.value = error
    }

    function loginSuccess(response) {
      authenticated.value = true
      notifyShow.value = true
      notifyKind.value = 'positive'
      notifyMsg.value = response
    }

    function loginError(error) {
      notifyShow.value = true
      notifyKind.value = 'negative'
      notifyMsg.value = error
    }

    function warning(error) {
      notifyShow.value = true
      notifyKind.value = 'warning'
      notifyMsg.value = error
    }

    return {
      connected,
      notifyShow,
      notifyKind,
      notifyMsg,
      connectionSuccess,
      connectionError,
      loginSuccess,
      loginError,
      warning,
    }
  }
}
</script>

<style lang="sass" scoped>
#logo
  width: 600px
</style>
