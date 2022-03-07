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
      class="bg-red text-white"
      flat
      dense
      label="LOCK DEVICES"
      @click="lockDevices"
      />
      <q-btn
      class="bg-green text-white"
      flat
      dense
      label="UNLOCK DEVICES"
      @click="unlockDevices"
      />
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

    <div class="q-pa-md row justify-between items-start q-gutter-md">
      <device-interface v-for="device in deviceList" 
        :device="device"
        :key="device.id"
        @configure-device="configureDeviceDialog"
      />
    </div>

    <device-edit 
      v-model:new="newDevice"
      v-model:device="selectedDevice"
      v-model:show="showDialog"
      @add-device="addDevice"
      @remove-device="removeDevice"
      @update-device="updateDevice"
    />

    <device-add
      v-model:show="showAddDialog"
      @new-device="configureDeviceDialog"
    />

    <notification v-model:show="notifyShow" :kind="notifyKind" :msg="notifyMsg"/>
  </q-page>
</template>

<script lang="ts">
import { ref, Ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import DeviceAdd from '@/components/DeviceAdd.vue'
import DeviceEdit from '@/components/DeviceEdit.vue'
import DeviceInterface from '@/components/DeviceInterface.vue'
import Notification from '@/components/Notification.vue'
import { Device } from '@/libs/device'

export default {
  name: 'Configure',
  components: {
    DeviceAdd,
    DeviceEdit,
    DeviceInterface,
    Notification,
  },
  setup: () => {
    const deviceList: Ref<Device[]> = ref([])
    const newDevice = ref(false)
    const notifyShow = ref(false)
    const notifyKind = ref('positive')
    const notifyMsg = ref('')
    const showDialog = ref(false)
    const showAddDialog = ref(false)
    const selectedDevice = ref(new Device)

    // pull device list from rust frontend
    getDeviceList()

    function lockDevices() {
      invoke("lock_devices")
        .then((response) => {
          deviceList.value = JSON.parse(response as string)
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

      window.location.reload()
    }

    function unlockDevices(){
      invoke("unlock_devices")
        .then((response) => {
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

    }

    // show the add device dialog window
    function addDeviceDialog() {
      newDevice.value = true
      showAddDialog.value = true
    }
    
    function addDevice(dev: Device) {
      const tempDev = new Device
      tempDev.clone(dev)

      invoke("add_device", { dev: JSON.stringify(tempDev) })
        .then((response) => {
          deviceList.value.push(tempDev)
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
      
      showDialog.value = false
      selectedDevice.value.clear()
    }

    // show the configure device dialog window
    function configureDeviceDialog(dev: Device, newDev: boolean) {
      selectedDevice.value.clone(dev)
      newDevice.value = newDev ? true : false
      showDialog.value = true
    }

    function getDeviceList() {
      invoke("get_device_list")
        .then((response) => {
          deviceList.value = JSON.parse(response as string)
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
    }

    function removeDevice(dev: Device) {
      invoke("remove_device", { dev: JSON.stringify(dev) })
        .then((response) => {
          deviceList.value.splice(deviceList.value.findIndex(el => el.id == dev.id), 1)
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

      showDialog.value = false
      selectedDevice.value.clear()
    }

    function updateDevice(dev: Device) {
      const tempDev = new Device
      tempDev.clone(dev)

      invoke("update_device", { dev: JSON.stringify(tempDev) })
        .then((response) => {
          deviceList.value[deviceList.value.findIndex(el => el.id == tempDev.id)] = tempDev
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
      
      showDialog.value = false
      selectedDevice.value.clear()
    }

    return {
      lockDevices,
      unlockDevices,
      addDeviceDialog,
      addDevice,
      configureDeviceDialog,
      deviceList,
      newDevice,
      notifyShow,
      notifyKind,
      notifyMsg,
      removeDevice,
      selectedDevice,
      showDialog,
      showAddDialog,
      updateDevice,
    }
  }
}
</script>
