<template>
  <q-page class="full-width column wrap" padding style="padding-top: 66px;">
    <q-page-sticky position="top" expand class="bg-grey text-white">
      <q-toolbar>
        <q-icon name="settings" size="md"/>
        <q-toolbar-title>Configure</q-toolbar-title>
      </q-toolbar>
    </q-page-sticky>

    <div class="fit row justify-end">
      <q-btn
        color="primary"
        flat
        dense
        icon-right="add_circle"
        label="ADD DEVICE"
        @click="addDeviceDialog"
      />
    </div>

    <q-separator />

    <div class="q-pa-md row items-start q-gutter-md">
      <device-interface v-for="device in deviceList" 
        :device="device"
        :key="device.name"
        @configure-device="configureDevice"
      />
    </div>

    <device-edit 
      v-model:new="newDevice"
      v-model:device="selectedDevice"
      v-model:show="showDialog"
      @add-device="addDevice"
    />
  </q-page>
</template>

<script lang="ts">
import { ref } from 'vue'
import DeviceInterface from '@/components/DeviceInterface.vue'
import DeviceEdit from '@/components/DeviceEdit.vue'
import { Device, Battery, Inverter, Sensor } from '@/libs/device'

export default {
  name: 'Configure',
  components: {
    DeviceInterface,
    DeviceEdit,
  },
  setup: () => {
    const deviceList: Array<Device> = [new Battery, new Inverter, new Sensor]
    const newDevice = ref(false)
    const showDialog = ref(false)
    const selectedDevice = ref<Device | null>(null)

    function addDeviceDialog() {
      newDevice.value = true
      showDialog.value = true
    }

    function configureDevice(dev: Device) {
      selectedDevice.value = dev
      newDevice.value = false
      showDialog.value = true
    }
    
    function addDevice(dev: Device) {
      deviceList.push(dev)
      showDialog.value = false
    }

    return {
      deviceList,
      newDevice,
      addDeviceDialog,
      showDialog,
      selectedDevice,
      configureDevice,
      addDevice,
    }
  }
}
</script>
