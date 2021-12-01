<template>
  <q-layout view="hHh Lpr lff">
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
      v-model="leftDrawerOpen"
      bordered
      elevated
      overlay
      class="bg-grey-2"
    >
      <q-list>
        <q-item clickable to="/dashboard">
          <q-item-section avatar>
            <q-icon name="leaderboard" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Dashboard</q-item-label>
            <q-item-label caption>Controls & Telemetry</q-item-label>
          </q-item-section>
        </q-item>
        <q-item clickable to="/configure">
          <q-item-section avatar>
            <q-icon name="settings" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Configure</q-item-label>
            <q-item-label caption>Add, Modify & Remove Devices</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />

        <q-item clickable to="/manage">
          <q-item-section avatar>
            <q-icon name="manage_accounts" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Manage</q-item-label>
            <q-item-label caption>Add, Modify & Remove Users</q-item-label>
          </q-item-section>
        </q-item>

        <q-separator />

        <q-item clickable to="/start">
          <q-item-section avatar>
            <q-icon name="logout" />
          </q-item-section>
          <q-item-section>
            <q-item-label>Logout</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { ref } from 'vue'

export default {
  name: 'LayoutDefault',

  components: {
  },

  setup () {
    return {
      leftDrawerOpen: ref(false),
      miniState: ref(true)
    }
  }
}

</script>
