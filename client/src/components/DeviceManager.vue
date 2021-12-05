<template>
    <div>
        <h1>Connected Devices</h1>
        <q-list bordered separator id="dynamic-list">
          <q-item>
            <DeviceInterface v-for="device in devices" 
              :device_type="device.type" 
              :name = "device.name" 
              :key="device.IP"
            />
          </q-item>
        </q-list>

        <label for="device-type">Choose Device Type</label>
        <select name="device-type" id="device-type">
          <option value="Battery">Battery</option>
          <option value="Inverter">Inverter</option>
          <option value="Custom">Custom</option>
        </select>


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

    //execute the function that allows for pinging devices
    getDeviceList();

    return {
      getDeviceList,
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
