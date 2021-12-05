<template>
    <div>
        <h1>Connected Devices</h1>
        <h4>In the final product, data will be obtained in realtime from the backend server (located in the Pod Computer)</h4>
        <q-list bordered separator id="dynamic-list">
          <q-item>
            <DeviceInterface v-for="device in devices" 
              :device_type="device.type" 
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

<script>
import DeviceInterface from './DeviceInterface.vue'

export default {
  name: 'DeviceManager',

  // define functions that can be called 
  methods: {
  },
  components: {
    DeviceInterface
  },
  // define data that can be referenced
  data(){
    return{
      devices:
      [
        {IP:"0", name:"Placeholder Device", type:"Battery"},
        {IP:"1", name:"Placeholder Device", type:"Inverter"},
      ]
    }
  }
}


</script>
