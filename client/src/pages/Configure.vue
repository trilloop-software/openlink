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
        @configure-device="configureDeviceDialog"
      />
    </div>

    <device-edit 
      v-model:new="newDevice"
      v-model:device="selectedDevice"
      v-model:show="showDialog"
      @add-device="addDeviceToPod"
      @modify-device="modifyPodDevice"
      @remove-device="removeDeviceFromPod"
    />

    <device-add
      v-model:show="showAddDialog"
      @new-device="configureDeviceDialog"
    />
  </q-page>
</template>

<script lang="ts">
import { ref } from 'vue'
import DeviceInterface from '@/components/DeviceInterface.vue'
import DeviceEdit from '@/components/DeviceEdit.vue'
import DeviceAdd from '@/components/DeviceAdd.vue'
import { Device, Battery, Inverter, Sensor } from '@/libs/device'

export default {
  name: 'Configure',
  components: {
    DeviceInterface,
    DeviceEdit,
    DeviceAdd,
  },
  setup: () => {
    const deviceList: Array<Device> = [new Battery, new Inverter, new Sensor]
    const newDevice = ref(false)
    const showDialog = ref(false)
    const showAddDialog = ref(false)
    const selectedDevice = ref<Device | null>(null)

    function addDeviceDialog() {
      newDevice.value = true
      showAddDialog.value = true
    }

    function configureDeviceDialog(dev: Device, newDev: boolean) {
      selectedDevice.value = dev
      newDevice.value = newDev ? true : false
      showDialog.value = true
    }
    
    // TODO: implement in rust frontend + backend
    function addDeviceToPod(dev: Device) {
      deviceList.push(dev)
      showDialog.value = false
    }

    // TODO: implement in rust frontend + backend
    function modifyPodDevice(dev: Device) {
      deviceList[deviceList.findIndex(el => el.id === dev.id)] = dev
      showDialog.value = false
    }

    // TODO: implement in rust frontend + backend
    function removeDeviceFromPod(dev: Device) {
      deviceList.splice(deviceList.findIndex(el => el.id === dev.id), 1)
      showDialog.value = false
    }

    return {
      deviceList,
      newDevice,
      addDeviceDialog,
      showDialog,
      showAddDialog,
      selectedDevice,
      configureDeviceDialog,
      addDeviceToPod,
      modifyPodDevice,
      removeDeviceFromPod,
    }
  }
}
</script>
