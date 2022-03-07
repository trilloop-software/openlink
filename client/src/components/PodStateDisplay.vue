<template>
  <div class="column items-center">
    <q-icon color="primary" :name="getPodStateIcon(podState)" size="8rem" />
    <span class="text-center text-h4 text-primary text-uppercase text-weight-medium">{{ podState }}</span>
  </div>
</template>

<script lang="ts">
import { PodState, PodStateIcon } from '@/types/podstate'
import { useModelWrapper } from '@/utils/modelWrapper'

export default {
  name: 'PodStateDisplay',
  props: {
    state: { type: String as () => PodState, default: PodState.Unlocked }
  },
  emits: ['update:state'],
  setup: (props: any, { emit }) => {
    function getPodStateIcon(podState: PodState) {
      return PodStateIcon[podState]
    }

    return {
      getPodStateIcon,
      podState: useModelWrapper(props, emit, 'state'),
    }
  }
}
</script>
