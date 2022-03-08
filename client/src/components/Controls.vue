<template>
  <div class="fit column items-center q-gutter-y-lg">
    <q-input v-model="distance" placeholder="Distance in meters" />
    <q-input v-model="maxSpeed" placeholder="Speed in km/h" />
    <q-btn class="same-length" :disabled="states.podState != PodState.Locked" push color="primary" label="Set Destination" @click="setDestination" />
    <q-btn class="same-length" :disabled="states.podState != PodState.Locked" push color="primary" label="Launch Pod" @click="launchPod" />
    <q-btn class="same-length" :disabled="states.podState != PodState.Moving" push color="primary" label="Stop Pod" @click="stopPod" />
  </div>
</template>

<style>
.same-length {
  width: 250px;
}
</style>

<script lang="ts">
import { ref } from 'vue'

import { PodState } from '@/types/podstate'
import { statesStore } from '@/stores/states'


export default {
  name: 'Controls',
  emits: ['update:state','launch-pod','set-destination','stop-pod'],
  setup: (props: any, { emit }) => {
    const states = statesStore()

    const distance = ref(undefined)
    const maxSpeed = ref(undefined)

    function launchPod() {
      emit('launch-pod')
    }

    function setDestination() {
      emit('set-destination', distance, maxSpeed)
    }

    function stopPod() {
      emit('stop-pod')
    }

    return {
      distance,
      launchPod,
      maxSpeed,
      PodState,
      setDestination,
      states,
      stopPod,
    }
  }
}
</script>
