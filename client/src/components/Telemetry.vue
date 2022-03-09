<template>
  <div v-if="states.podState == PodState.Unlocked" class="column items-center q-mt-xl">
    <span class="text-grey-3 text-h1 text-weight-bold">N/A</span>
  </div>
  <div v-else>
    <h2>Telemetry Data</h2>
      <v-simple-table id = "telemetry" width="100%">
          <thead>
            <tr>
              <th class="text-left">
                Category
              </th>
              <th class="text-left">
                Value
              </th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="item in data"
              :key="item.value"
            >
              <td>{{ item.field_name }}</td>
              <td>{{ item.field_value }}</td>
            </tr>
          </tbody>
      </v-simple-table>
  </div>
</template>

<style>

#telemetry {
  font-family: Arial, Helvetica, sans-serif;
  border-collapse: collapse;
  width: 100%;
  table-layout: fixed;
  display: table;
}

#telemetry td, #telemetry th {
  border: 1px solid #ddd;
  padding: 8px;
}

#telemetry tr:nth-child(even){background-color: #f2f2f2;}

#telemetry tr:hover {background-color: #ddd;}

#telemetry th {
  padding-top: 12px;
  padding-bottom: 12px;
  text-align: left;
  background-color: #4F2683;
  color: white;
}

</style>

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
    const states = statesStore()
    const tele_data = [
      {category: 'Accelerometer', value: 0},
      {category: 'Brake Temperature', value: 50},
      {category: 'Battery Temperature', value: 30},
      {category: "Battery Current", value: 12}
    ]

    getTelemetry()
    setInterval(getTelemetry, 1000)

    function getTelemetry() {
      if (states.podState != PodState.Unlocked) {
        emit('get-telemetry')
      }
    }

    return {
      getTelemetry,
      PodState,
      states,
      tele_data,
    }
  },
}

</script>
