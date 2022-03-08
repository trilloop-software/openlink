<template>
  <div class="fit column items-center q-gutter-y-lg">
    <q-input
      class="q-pt-xl"
      dense
      filled
      v-model.number="distance"
      label="Distance (meters)"
    />
    <q-input
      dense
      filled
      v-model.number="maxSpeed"
      label="Speed (KM/H)"
    />
    <q-btn
      class="same-length"
      :disabled="states.podState != PodState.Locked"
      push
      color="primary"
      label="Set Destination"
      @click="setDestination"
    />
    <q-btn
      class="same-length"
      :disabled="states.podState != PodState.Locked"
      push
      color="primary"
      label="Launch Pod"
      @click="launchPod"
    />
    <q-btn
      class="same-length"
      :disabled="states.podState != PodState.Moving"
      push
      color="primary"
      label="Stop Pod"
      @click="stopPod"
    />
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
  emits: ['update:state','launch-pod','params-warning','set-destination','stop-pod'],
  setup: (props: any, { emit }) => {
    const states = statesStore()

    const distance = ref(undefined)
    const maxSpeed = ref(undefined)

    function launchPod() {
      emit('launch-pod')
    }

    function setDestination() {
      if (isNaN(distance.value)) {
        emit('params-warning', 'Please enter a valid distance')
      } else {
        if (isNaN(maxSpeed.value)) {
          emit('params-warning', 'Please enter a valid max speed')
        } else {
          emit('set-destination', distance, maxSpeed)
        }
      }
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
