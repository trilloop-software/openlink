<template>
  <div class="column items-center">
    <q-icon color="primary" :name="getPodStateIcon(states.podState)" size="8rem" />
    <span class="text-center text-h4 text-primary text-uppercase text-weight-medium">{{ states.podState }}</span>
  </div>
</template>

<script lang="ts">
import { PodState, PodStateIcon } from '@/types/podstate'
import { statesStore } from '@/stores/states'

export default {
  name: 'PodStateDisplay',
  setup: () => {
    const states = statesStore()

    function getPodStateIcon(podState: PodState) {
      return PodStateIcon[podState]
    }

    return {
      getPodStateIcon,
      states
    }
  }
}
</script>