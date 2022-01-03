import { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { Device } from '@/libs/device'

export function getDeviceList(deviceList: Ref<Device[]>) {
  invoke("get_device_list")
    .then((response) => {
      deviceList.value = JSON.parse(response as string)
    })
    .catch((error) => {
      alert('Error:' + error);
    })
}