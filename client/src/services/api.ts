import { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { Device } from '@/libs/device'

export function addDevice(device: Device) {  
  invoke("add_device", { dev: JSON.stringify(device) })
    .then((response) => {
      alert(response)
    })
    .catch((error) => {
      alert('Error:' + error)
    })
}

export function getDeviceList(deviceList: Ref<Device[]>) {
  invoke("get_device_list")
    .then((response) => {
      deviceList.value = JSON.parse(response as string)
    })
    .catch((error) => {
      alert('Error:' + error)
    })
}

export function removeDevice(device: Device) {
  invoke("remove_device", { dev: JSON.stringify(device) })
  .then((response) => {
    alert(response)
  })
  .catch((error) => {
    alert('Error:' + error)
  })
}

export function updateDevice(device: Device) {
  console.log(device)
  console.log(JSON.stringify(device))
  invoke("update_device", { dev: JSON.stringify(device) })
    .then((response) => {
      alert(response)
    })
    .catch((error) => {
      alert('Error:' + error)
    })
}