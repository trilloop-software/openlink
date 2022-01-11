<template>
  <q-card class="device-card" flat bordered>
    <q-item>
      <q-card-section avatar>
        <q-icon color="primary" :name="getDeviceIcon(device.device_type)" size="lg" />
      </q-card-section>

      <q-card-section>
        <q-item-label>{{ device.name }}</q-item-label>
        <q-item-label caption class="text-uppercase">{{ device.device_type }}</q-item-label>
      </q-card-section>

      <q-space />

      <q-card-actions>
        <q-btn v-if="device.connection_status == 'Connected' && device.device_status == 'Operational'" 
          color="green"
          round
          flat
          dense
          icon="check_circle"
          @click="deviceDiagnostics(device)"
        >
          <q-tooltip>
            Device is connected and operational
          </q-tooltip>
        </q-btn>

        <q-btn v-else              
          color="red"
          round
          flat
          dense
          icon="warning"
          @click="deviceDiagnostics(device)"
        >
          <q-tooltip>
            Device is <span class="text-lowercase">{{ device.connection_status }}</span> and
            <span class="text-lowercase">{{ device.device_status }}</span>
          </q-tooltip>
        </q-btn>

        <q-btn
          color="grey"
          round
          flat
          dense
          icon="settings"
          @click="deviceConfigure(device)"
        />

        <q-btn
          color="grey"
          round
          flat
          dense
          :icon="expanded ? 'keyboard_arrow_up' : 'keyboard_arrow_down'"
          @click="expanded = !expanded"
        />
      </q-card-actions>
    </q-item>

    <q-slide-transition>
      <div v-show="expanded">
        <q-separator />

        <q-card-section>
          <q-list dense separator>
            <q-item class="fit row justify-between">
              <q-item-section class="text-weight-bold">IP Address</q-item-section>
              <q-item-section>{{ device.ip_address }}:{{ device.port }}</q-item-section>
            </q-item>

            <q-item>
              <q-item-section class="text-weight-bold">Connection Status</q-item-section>
              <q-item-section>{{ device.connection_status }}</q-item-section>
            </q-item>

            <q-item>
              <q-item-section class="text-weight-bold">Device Status</q-item-section>
              <q-item-section>{{ device.device_status }}</q-item-section>
            </q-item>

            <q-item v-for="f in device.fields" :key="f.field_name">
              <q-item-section class="text-weight-bold">{{ f.field_name }}</q-item-section>
              <q-item-section>{{ f.field_value }}</q-item-section>
            </q-item>
          </q-list>
        </q-card-section>
      </div>
    </q-slide-transition>
  </q-card>
</template>

<script lang="ts">
import { ref, PropType } from 'vue'
import { Device, getDeviceIcon } from '@/libs/device'

export default {
  name: 'DeviceInterface',
  props: {
    device: { type: Object as PropType<Device>, default: new Device }
  },
  emits: ['configure-device', 'device-diagnostics'],
  setup: (props: any, { emit }) => {
    function deviceConfigure(dev: Device) {
      emit('configure-device', dev, false)
    }

    function deviceDiagnostics(dev: Device) {
      emit('device-diagnostics', dev)
    }

    return {
      expanded: ref(false),
      deviceConfigure,
      deviceDiagnostics,
      getDeviceIcon
    }
  }
}
</script>

<style lang="sass" scoped>
.device-card
  width: 100%
  max-width: 460px

.error
  color: 'red'
</style>
