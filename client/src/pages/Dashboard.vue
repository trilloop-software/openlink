<template>
  <q-page class="full-width column wrap" padding style="padding-top: 66px;">
    <q-page-sticky position="top" expand class="bg-grey text-white">
      <q-toolbar>
        <q-icon name="leaderboard" size="md"/>
        <q-toolbar-title>Dashboard</q-toolbar-title>
      </q-toolbar>
    </q-page-sticky>

    <div class="row">
      <div class="col q-pt-md">
        <pod-state-display v-model:state="podState" />
        <controls v-model:state="podState" @launch-pod="launchPod" @set-destination="setDestination" @stop-pod="stopPod" />
      </div>
      <div class="col">
        <telemetry />
      </div>
    </div>

    <notification v-model:show="notifyShow" :kind="notifyKind" :msg="notifyMsg" />
  </q-page>
</template>

<script lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import Controls from '@/components/Controls.vue'
import Notification from '@/components/Notification.vue'
import PodStateDisplay from '@/components/PodStateDisplay.vue'
import Telemetry from '@/components/Telemetry.vue'
import { PodState } from '@/types/podstate'

export default {
  name: 'Dashboard',
  components: {
    Controls,
    Notification,
    PodStateDisplay,
    Telemetry,
  },
  setup: () => {
    const distance = ref(undefined)
    const maxSpeed = ref(undefined)
    const podState = ref(PodState.Unlocked)

    const notifyShow = ref(false)
    const notifyKind = ref('positive')
    const notifyMsg = ref('')

    getPodState()

    function setDestination() {
      invoke("set_destination", { params: JSON.stringify({ distance: distance.value, max_speed: maxSpeed.value })})
        .then((response) => {
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        }).catch((error) =>{
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
    }

    function launchPod() {
      invoke("launch")
        .then((response) => {
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        }).catch((error) =>{
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

      getPodState()
    }

    function stopPod() {
      invoke("stop")
        .then((response) => {
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        }).catch((error) =>{
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

      getPodState()
    }

    function getPodState(){
      invoke("get_pod_state")
        .then((response) => {
          if (response == "\"Unlocked\"") {
            podState.value = PodState.Unlocked
          }
          else if (response == "\"Locked\"") {
            podState.value = PodState.Locked
          }
          else if (response == "\"Moving\"") {
            podState.value = PodState.Moving
          }
          else if (response == "\"Braking\"") {
            podState.value = PodState.Braking
          }
        }).catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
    }

    return {
      distance,
      getPodState,
      launchPod,
      maxSpeed,
      notifyShow,
      notifyKind,
      notifyMsg,
      podState,
      setDestination,
      stopPod,
    }
  }
}
</script>
