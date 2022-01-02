<template>
  <q-dialog v-model="showDialog" persistent>
    <q-card>
      <q-bar>
        <q-item-label class="text-weight-bold">Add Device</q-item-label>
        <q-space />
        <q-btn flat icon="close" v-close-popup />
      </q-bar>

      <q-card-section>
        <q-btn-toggle
          v-model="newDeviceType"
          toggle-color="primary"
          toggle-text-color="white"
          size="2rem"
          :options="newDeviceTypeOptions"
        >
          <template v-slot:Battery>
            <div class="column items-center text-caption">
              <q-icon :name="DeviceTypeIcon.Battery" size="2rem"/>
              <q-item-label>BATTERY</q-item-label>
            </div>
          </template>
          <template v-slot:Inverter>
            <div class="column items-center text-caption">
              <q-icon :name="DeviceTypeIcon.Inverter" size="2rem" />
              <q-item-label>INVERTER</q-item-label>
            </div>
          </template>
          <template v-slot:Sensor>
            <div class="column items-center text-caption">
              <q-icon :name="DeviceTypeIcon.Sensor" size="2rem" />
              <q-item-label>SENSOR</q-item-label>
            </div>
          </template>
        </q-btn-toggle>
      </q-card-section>

      <q-separator />

      <q-btn 
        class="fit row content-end"
        flat
        color="primary"
        label="Device Settings"
        icon-right="arrow_forward_ios"
        @click="configureNewDevice"
      />
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { ref } from 'vue'
import { DeviceType, DeviceTypeIcon, Battery, Inverter, Sensor } from '@/libs/device'
import { useModelWrapper } from '@/utils/modelWrapper'

export default {
  name: 'DeviceAdd',
  props: {
    show: { type: Boolean, default: false }
  },
  emits: ['update:show','new-device'],
  setup: (props: any, { emit }) => {
    const newDeviceType = ref(DeviceType.Battery)
    const newDeviceTypeOptions = [
      { value: DeviceType.Battery, slot: 'Battery' },
      { value: DeviceType.Inverter, slot: 'Inverter' },
      { value: DeviceType.Sensor, slot: 'Sensor' }
    ]

    function configureNewDevice() {
      emit('update:show', false)
      if (newDeviceType.value === DeviceType.Battery) {
        emit('new-device', new Battery, true)
      } else if (newDeviceType.value === DeviceType.Inverter) {
        emit('new-device', new Inverter, true)
      } else {
        emit('new-device', new Sensor, true)
      }
    }

    return {
      showDialog: useModelWrapper(props, emit, 'show'),
      DeviceTypeIcon,
      newDeviceType,
      newDeviceTypeOptions,
      configureNewDevice,
    }
  }
}
</script>
