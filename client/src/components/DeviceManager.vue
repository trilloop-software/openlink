<template>
    <div>
      <h2>Device Manager</h2>

      <h4>Connected Devices</h4>
      <q-list bordered separator dense id="dynamic-list">

        <DeviceInterface v-for="device in devices" 
          :device_type="device.type" 
          :name = "device.name" 
          :key="device.IP"
        />

      </q-list>

      <h4>Add New Device</h4>
      <q-input v-model="text" label="IP Address" id="device-ip"/>
      <label for="device-type">Choose Device Type</label>
      <select name="device-type" id="device-type">
        <option value="Battery">Battery</option>
        <option value="Inverter">Inverter</option>
        <option value="Custom">Custom</option>
      </select>
      <q-btn push color="primary" label="Add" @click="addDevice()" />

    </div>

</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import DeviceInterface from './DeviceInterface.vue'

export default {
  name: 'DeviceManager',

  /**
   * define necessary data structures and etc
   * such as functions that can be called 
   * such as data that can be referenced
   */
  setup: () => {

    //define the function that allows for retrieving devices
    function getDeviceList(){

      invoke("get_device_list")
        .then((response) => {
          alert('Successful: ' + response)
        })
        .catch((error) => {
          alert('Error:' + error)
        })
    }

    function addDevice(){

      let type = document.getElementById("device-type") as HTMLInputElement;

      invoke("add_device",{deviceType:type.value})
        .then((response) => {
          alert('Successful: ' + response)
        })
        .catch((error) => {
          alert('Error:' + error)
        })
    }

    //execute the function that allows for pinging devices
    getDeviceList();

    return {
      getDeviceList,
      addDevice,
    }
  },
  // define data that can be referenced
  data(){
    return{

      //Placholder device data until we can figure out parsing packets received from backend
      devices:
      [
        {IP:"0", name:"Placeholder Battery", type:"Battery"},
        {IP:"1", name:"Placeholder Inverter", type:"Inverter"},
      ]
    }
  },
  components: {
    DeviceInterface
  },
}


</script>
