import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

export const statesStore = defineStore({
  id: 'states',
  state: () => ({
    connectState: false,
    loginState: false,
  }),
  actions: {
    getConnectionState() {
      invoke('check_conn')
        .then(() => this.connectState = true)
        .catch(() => this.connectState = false)
    },
    getLoginState() {
      invoke('check_auth')
      .then(() => this.loginState = true)
      .catch(() => this.loginState = false)
    },
  }
})