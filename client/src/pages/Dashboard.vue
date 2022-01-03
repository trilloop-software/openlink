<template>
  <q-page class="full-width column wrap" padding style="padding-top: 66px;">
    <q-page-sticky position="top" expand class="bg-grey text-white">
      <q-toolbar>
        <q-icon name="leaderboard" size="md"/>
        <q-toolbar-title>Dashboard</q-toolbar-title>
      </q-toolbar>
    </q-page-sticky>

    <div class="fit column items-center q-gutter-y-lg">
      <q-btn class = "same-length" push color="primary" label="Set Course Destination" @click= "setDestination"/>
      <q-btn class = "same-length" push color="primary" label="Launch Pod" @click= "launch"/>
      <q-btn class = "same-length" push color="primary" label="Stop Pod" @click= "stop"/>
      <q-btn class = "same-length" push color="primary" label="Emergency Stop" @click= "emergencyStop"/>
    </div>
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
    function setDestination() {
    invoke("set_destination").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    function launch() {
    invoke("launch").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    function stop() {
      invoke("stop").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
    }

    function emergencyStop() {
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
