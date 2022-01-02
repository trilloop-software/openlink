<template>
  <q-card class="device-card" flat bordered>
    <q-item>
      <q-card-section avatar>
        <q-icon color="primary" :name="device.icon" size="lg" />
      </q-card-section>

      <q-card-section>
        <q-item-label>{{ device.name }}</q-item-label>
        <q-item-label caption>{{ device.device_type }}</q-item-label>
      </q-card-section>

      <q-space />

      <q-card-actions>
        <q-btn v-if="device.connection_status && device.device_status" 
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
            Device is {{ device.connection_status ? 'connected' : 'disconnected' }} and
            {{ device.device_status ? 'operational' : 'unsafe' }}
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
              <q-item-section>{{ device.connection_status ? 'Connected' : 'Disconnected' }}</q-item-section>
            </q-item>

            <q-item>
              <q-item-section class="text-weight-bold">Device Status</q-item-section>
              <q-item-section>{{ device.device_status ? 'Operational' : 'Unsafe' }}</q-item-section>
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
import { Device } from '@/libs/device'

export default {
  name: 'DeviceInterface',
  props: {
    device: { type: Object as PropType<Device> }
  },
  emits: ['configure-device', 'device-diagnostics'],
  setup: (props: any, context: any) => {
    function deviceConfigure(dev: Device) {
      context.emit('configure-device', dev)
    }

    function deviceDiagnostics(dev: Device) {
      context.emit('device-diagnostics', dev)
    }

    return {
      expanded: ref(false),
      deviceConfigure,
      deviceDiagnostics,
    }
  }
}
</script>

<style lang="sass" scoped>
.device-card
  width: 100%
  max-width: 450px

.error
  color: 'red'
</style>
