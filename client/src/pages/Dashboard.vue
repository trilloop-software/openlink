<template>
  <q-page class="flex flex-center column q-gutter-y-lg">
    <h2>Dashboard</h2>
    <q-btn class = "same-length" push color="primary" label="Set Course Destination" @click= "setDestination"/>
    <q-btn class = "same-length" push color="primary" label="Launch Pod" @click= "launch"/>
    <q-btn class = "same-length" push color="primary" label="Stop Pod" @click= "stop"/>
    <q-btn class = "same-length" push color="primary" label="Emergency Stop" @click= "emergencyStop"/>
  </q-page>
</template>

<style>
.same-length {
  width: 250px;
}
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
export default {
  name: 'Dashboard',
  setup: () => {
    function setDestination()
    {
    invoke("set_destination").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    function launch()
    {
    invoke("launch").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    function stop()
    {
      invoke("stop").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    function emergencyStop()
    {
    invoke("emergency_stop").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    return {setDestination, launch, stop, emergencyStop}
  }
}
</script>
