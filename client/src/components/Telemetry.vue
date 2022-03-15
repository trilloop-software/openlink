<template>
  <div v-if="states.podState == PodState.Unlocked" class="column items-center q-mt-xl">
    <span class="text-grey-3 text-h1 text-weight-bold">N/A</span>
  </div>
  <div v-else>
    <q-table
      class="q-mt-md"
      title="Telemetry"
      :rows="data"
      :columns="col"
    />
  </div>
</template>

<script lang="ts">
import { PodState } from '@/types/podstate'
import { statesStore } from '@/stores/states'

export default {
  name: 'Telemetry',
  props: {
    data: { default: [] }
  },
  emits: ['get-telemetry'],
  setup: (props: any, { emit }) => {
    const col = [
      {
        name: 'field_name',
        label: 'Metric',
        align: 'left',
        field: 'field_name',
      },
      {
        name: 'field_value',
        label: 'Value',
        align: 'right',
        field: 'field_value',
      },
      {
        name: 'value_upper',
        label: 'Upper Bound',
        align: 'right',
        field: 'value_upper'
      },
      {
        name: 'value_lower',
        label: 'Lower Bound',
        align: 'right',
        field: 'value_lower'
      },
    ]
    const states = statesStore()

    getTelemetry()
    setInterval(getTelemetry, 1000)

    function getTelemetry() {
      if (states.podState != PodState.Unlocked) {
        emit('get-telemetry')
      }
    }

    return {
      col,
      getTelemetry,
      PodState,
      states,
    }
  },
}

</script>
