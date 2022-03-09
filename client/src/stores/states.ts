import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'
import { PodState } from '@/types/podstate'

export const statesStore = defineStore({
  id: 'states',
  state: () => ({
    connectState: false,
    loginState: false,
    podState: PodState.Unlocked,
  }),
  actions: {
    changePodState(response) {
      if (response == "\"Unlocked\"") {
        this.podState = PodState.Unlocked
      }
      else if (response == "\"Locked\"") {
        this.podState = PodState.Locked
      }
      else if (response == "\"Moving\"") {
        this.podState = PodState.Moving
      }
      else if (response == "\"Braking\"") {
        this.podState = PodState.Braking
      }
    },
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
    getPodState() {
      if (this.connectState && this.podState) {
        invoke('get_pod_state')
          .then((response) => this.changePodState(response))
          .catch((error) => alert(error))
      }
      
    },
  }
})