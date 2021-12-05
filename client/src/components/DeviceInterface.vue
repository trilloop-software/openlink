<template>
    <q-item class="dense justify-center items-left">
        <h3> {{name}} </h3>
        <q-btn push color="primary" label="Ping Device" @click="pingDevice(name)" />

        <q-list>

            <q-item class = "generic-field">
                <label for="ip_address">IP Address: </label>
                <input name="ip_address">
            </q-item>

            <q-item class = "generic-field">
                <label for="conection_metric">Metric for Connection Strength: </label>
                <label name="conection_metric">______</label>
            </q-item>

            <q-item class = "generic-field">
                <label for="connection_status">Connection Status: </label>
                <select name="connection_status" id="connection_status">
                <option value="Safe">Safe (above safety threshold)</option>
                <option value="Unsafe">Unsafe (below safety threshold)</option>
                <option value="Offline">Offline (disconnected/not found)</option>
                </select>
            </q-item>

            <q-item class = "device-specific-field" v-for="field in getFields(device_type)" :key="field">
                {{field}}: 
            </q-item>
        </q-list>

    </q-item>

</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
//import { ref } from 'vue'

export default {
  name: 'DeviceInterface',

  /**
   * define necessary data structures and etc
   * such as functions that can be called 
   * such as data that can be referenced
   */
  setup: () => {

    function pingDevice(name){

      invoke("ping_device",{name:name})
        .then((response) => {
          alert('Successful: ' + response)
        })
        .catch((error) => {
          alert('Error:' + error)
        })
    }

    var battery_fields: string[] = ["Temperature", "Power"];
    var inverter_fields: string[] = ["Inverter Field 1", "Inverter Field 2"];

    function getFields(type){
        let fields: string[] = [];
        switch(type){
            case "Battery":
                fields = battery_fields;
                break;
            case "Inverter":
                fields = inverter_fields;
                break;
        }
        return fields;
    }



    return {
        pingDevice,
        getFields,
    }
  },
  
  //parameters that can be passed in from external sources
  props: {
    name:String,
    device_type: String,
  },

}



</script>

<style>
    .generic-field{
        text-align: left;
        border-style: solid;
        border-color: #460d86;
    }
    .device-specific-field{
        text-align: left;
        border-style: solid;
        border-color: #9c9c9c;
    }
</style>