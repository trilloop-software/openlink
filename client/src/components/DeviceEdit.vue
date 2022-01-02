<template>
  <q-dialog v-model="showDialog" persistent>
    <q-card>
      <q-bar>
        <q-item-label class="text-weight-bold">Add Device</q-item-label>
        <q-space />
        <q-btn flat icon="close" v-close-popup />
      </q-bar>

      <div v-if="newDevice" class="q-pt-none">
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
      </div>

      <div v-else>
        <div class="fit row wrap justify-center">
          <q-card-section class="column justify-center items-center">
            <q-icon color="primary" :name="device.icon" size="4rem" />
            <q-item-label caption>{{ device.device_type }}</q-item-label>
          </q-card-section>
          <q-card-section class="q-gutter-y-md">
            <q-input
              dense
              filled
              v-model="configDevice.name"
              label="Name"
            />
            <q-input
              dense
              filled
              v-model="configDevice.ip_address"
              label="IP Address"
            />
            <q-input
              dense
              filled
              v-model="configDevice.port[0]"
              label="Port"
            />
          </q-card-section>
        </div>
        <div>
          <q-card-actions class="fit row justify-between">
            <q-btn
              color="primary"
              flat
              dense
              icon="settings_ethernet"
              label="PING"
              @click="configDevice.ping(configDevice.name, configDevice.ip_address, configDevice.port)"
            />

            <!-- <q-btn v-if="newDevice" -->
            <q-btn
              color="primary"
              flat
              dense
              icon-right="add"
              label="ADD"
              @click="addDevice(configDevice)"
            />

            <!-- TODO: UPDATE EXISTING DEVICES
            <q-btn v-else
              color="primary"
              flat
              dense
              icon-right="arrow_upward"
              label="UPDATE"
              @click="addDevice(configDevice)"
            />
            -->
          </q-card-actions>
        </div>
      </div>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { PropType, ref } from '@vue/runtime-core'
import { Device, DeviceType, DeviceTypeIcon, Battery, Inverter, Sensor } from '@/libs/device'
import { useModelWrapper } from '@/utils/modelWrapper'

export default {
  name: 'DeviceEdit',
  props: {
    new: { type: Boolean, default: false },
    device: { type: Object as PropType<Device> },
    show: { type: Boolean, default: false }
  },
  emits: ['update:show','update:device','update:new','add-device','update-device'],
  setup: (props: any, { emit }) => {
    // toggle buttons for device type
    const newDeviceType = ref(DeviceType.Battery)
    const newDeviceTypeOptions = [
      { value: DeviceType.Battery, slot: 'Battery' },
      { value: DeviceType.Inverter, slot: 'Inverter' },
      { value: DeviceType.Sensor, slot: 'Sensor' }
    ]

    // create new device and configure
    function configureNewDevice() {
      emit('update:new', false)
      if (newDeviceType.value === DeviceType.Battery) {
        emit('update:device', new Battery)
      } else if (newDeviceType.value === DeviceType.Inverter) {
        emit('update:device', new Inverter)
      } else {
        emit('update:device', new Sensor)
      }
    }

    // emit new device to parent component to add to device list
    // need to implement in backend
    function addDevice(dev: Device) {
      emit('add-device', dev)
    }

    return {
      DeviceTypeIcon,
      newDeviceType,
      newDeviceTypeOptions,
      configureNewDevice,
      addDevice,
      showDialog: useModelWrapper(props, emit, 'show'),
      newDevice: useModelWrapper(props, emit, 'new'),
      configDevice: useModelWrapper(props, emit, 'device')
    }
  }
}
</script>
