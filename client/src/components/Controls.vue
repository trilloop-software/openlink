<template>
  <div class="fit column items-center q-gutter-y-lg">
    <q-input v-model="distance" placeholder="Example: 100"/>
    <q-input v-model="maxSpeed" />
    <q-btn class="same-length" :disabled="podState != PodState.Locked" push color="primary" label="Set Destination" @click="setDestination" />
    <q-btn class="same-length" :disabled="podState != PodState.Locked" push color="primary" label="Launch Pod" @click="launchPod" />
    <q-btn class="same-length" :disabled="podState != PodState.Moving" push color="primary" label="Stop Pod" @click="stopPod" />
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
import { useModelWrapper } from '@/utils/modelWrapper'

export default {
  name: 'Controls',
  props: {
    state: { type: String as () => PodState, default: PodState.Unlocked }
  },
  emits: ['update:state','launch-pod','set-destination','stop-pod'],
  setup: (props: any, { emit }) => {
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
      podState: useModelWrapper(props, emit, 'state'),
      PodState,
      setDestination,
      stopPod,
    }
  }
}
</script>
