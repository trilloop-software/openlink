<template>
    <div>
        <h2>Placeholder {{device_type}}</h2>

        <div class = "generic-field">
            <label for="ip_address">IP Address: </label>
            <input name="ip_address">
        </div>

        <div class = "generic-field">
            <label for="conection_metric">Metric for Connection Strength: </label>
            <label name="conection_metric">______</label>
        </div>

        <div class = "generic-field">
            <label for="connection_status">Connection Status: </label>
            <select name="connection_status" id="connection_status">
            <option value="Safe">Safe (above safety threshold)</option>
            <option value="Unsafe">Unsafe (below safety threshold)</option>
            <option value="Offline">Offline (disconnected/not found)</option>
            </select>
        </div>

        <div class = "device-specific-field" v-for="field in getFields(device_type)" :key="field">
            {{field}}: 
        </div>

    </div>

</template>

<script>
export default {
  name: 'DeviceInterface',

  // define functions that can be called 
  methods: {
    getFields(type){
        let fields = [];
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
  },
  //parameters that can be passed in from external sources
  props: {
    device_type: String
  },
    // define data that can be referenced
  data(){
    return{
        battery_fields,
        inverter_fields,
    }
  }
}

var battery_fields = ["Temperature", "Power"];
var inverter_fields = ["Inverter Field 1", "Inverter Field 2"];

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