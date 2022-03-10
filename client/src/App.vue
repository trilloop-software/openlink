<template>
  <q-layout view="hHh Lpr lff" class="disable-select">
    <q-header elevated bordered>
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          @click="leftDrawerOpen = !leftDrawerOpen"
          aria-label="Menu"
          icon="menu"
        />

        <q-toolbar-title>
          OpenLink
        </q-toolbar-title>

        <div>OpenLink v0.1.0</div>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-if="states.connectState"
      v-model="leftDrawerOpen"
      bordered
      elevated
      overlay
      class="bg-grey-2"
    >
      <q-list>
        <q-item v-if="states.loginState && (states.usergroupState == 1 || states.usergroupState == 255)" clickable to="/dashboard">
          <q-item-section avatar>
            <q-icon name="leaderboard" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Dashboard</q-item-label>
            <q-item-label caption>Controls & Telemetry</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />
        
        <q-item v-if="states.loginState && (states.usergroupState == 2 || states.usergroupState == 255)" clickable to="/configure">
          <q-item-section avatar>
            <q-icon name="settings" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Configure</q-item-label>
            <q-item-label caption>Add, Modify & Remove Devices</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />

        <q-item v-if="states.loginState && states.usergroupState == 255" clickable to="/manage">
          <q-item-section avatar>
            <q-icon name="manage_accounts" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Manage</q-item-label>
            <q-item-label caption>Add, Modify & Remove Users</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />
        
        <q-item clickable to="/start" @click="logoutUser">
          <q-item-section avatar>
            <q-icon :name="states.loginState ? 'logout' : 'login'" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{states.loginState ? 'Logout' : 'Login'}}</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />

        <q-item clickable to="/start" @click="disconnectPod">
          <q-item-section avatar>
            <q-icon name="link_off" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Disconnect</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
  
  <notification v-model:show="notifyShow" :kind="notifyKind" :msg="notifyMsg" />
</template>

<script lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import Notification from '@/components/Notification.vue'
import { statesStore } from '@/stores/states'

export default {
  name: 'LayoutDefault',

  components: {
    Notification,
  },

  setup () {
    const states = statesStore()
    
    states.getConnectionState()
    states.getLoginState()

    const notifyShow = ref(false)
    const notifyKind = ref('positive')
    const notifyMsg = ref('')

    function disconnectPod() {
      if (states.connectState) {
        invoke('disconnect')
          .then((response) => {
            states.connectState = false
            states.loginState = false
            states.usergroupState = 0
            notifyShow.value = true
            notifyKind.value = 'positive'
            notifyMsg.value = response as string
          })
          .catch((error) => {
            notifyShow.value = true
            notifyKind.value = 'negative'
            notifyMsg.value = error as string
          })
      }
    }

    function logoutUser() {
      if (states.loginState) {
        invoke('logout')
          .then((response) => {
            states.loginState = false
            states.usergroupState = 0
            notifyShow.value = true
            notifyKind.value = 'positive'
            notifyMsg.value = response as string
          })
          .catch((error) => {
            notifyShow.value = true
            notifyKind.value = 'negative'
            notifyMsg.value = error as string
          })
      }
    }

    return {
      disconnectPod,
      leftDrawerOpen: ref(false),
      logoutUser,
      miniState: ref(true),
      notifyShow,
      notifyKind,
      notifyMsg,
      states,
    }
  }
}

</script>
