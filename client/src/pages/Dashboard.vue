<template>
  <q-page class="flex flex-center">
    <button v-on:click= "stop">Stop pod</button>
    <button style="color: red" v-on:click= "emergencyStop">Emergency Stop</button>
  </q-page>
</template>

<style>
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
export default {
  name: 'Dashboard',
  setup: () => {
    function stop()
    {
      invoke("stop").then((response) => {
        alert(response as String);
      }).catch((err) =>{
        alert(err as String);
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
    return {stop, emergencyStop}
  }
}
</script>
