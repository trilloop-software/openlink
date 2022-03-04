<template>
  <q-page class="full-width column wrap" padding style="padding-top: 66px;">
    <q-page-sticky position="top" expand class="bg-grey text-white">
      <q-toolbar>
        <q-icon name="leaderboard" size="md"/>
        <q-toolbar-title>Dashboard</q-toolbar-title>
      </q-toolbar>
    </q-page-sticky>
<label id = "podStat"></label>
<q-btn class = "same-length" push color="primary" label="Refresh" @click= "getPodState"/>
    <div class="fit column items-center q-gutter-y-lg">
      <q-input id = "time-distance" placeholder="Example: 100000"/>
      <q-btn class = "same-length" id = "destin" push color="primary" label="Set Course Destination" @click= "setDestination"/>
      <q-btn class = "same-length" id = "launch" push color="primary" label="Launch Pod" @click= "launch"/>
      <q-btn class = "same-length" id = "stop" push color="primary" label="Stop Pod" @click= "stop"/>
      <q-btn class = "same-length" id = "estop" push color="primary" label="Emergency Stop" @click= "emergencyStop"/>
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
    getPodState()
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
      getPodState();
    }

    function stop() {
      invoke("stop").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
      getPodState();
    }

    function emergencyStop() {
    invoke("emergency_stop").then((response) => {
        alert("Success: " + response);
      }).catch((err) =>{
        alert("Error: " + err);
      });
      getPodState();
    }
    function getPodState(){
      invoke("get_pod_state").then((response) => {
        //response = "\"Locked\""; // manually testing
        document.getElementById('podStat').innerHTML = ("Pod Status: " + response);
        if(response == "\"Unlocked\""){
          document.getElementById("destin").style.display = 'none';
          document.getElementById("launch").style.display = 'none';
          document.getElementById("stop").style.display = 'none';
          document.getElementById("estop").style.display = 'none';
        }
        else if(response == "\"Locked\""){
          document.getElementById("destin").style.display = 'block';
          document.getElementById("launch").style.display = 'block';
          document.getElementById("stop").style.display = 'none';
          document.getElementById("estop").style.display = 'none';
        }
        else if(response == "\"Moving\""){
          document.getElementById("destin").style.display = 'none';
          document.getElementById("launch").style.display = 'none';
          document.getElementById("stop").style.display = 'block';
          document.getElementById("estop").style.display = 'block';
        }
        else if(response == "\"Braking\""){
          document.getElementById("destin").style.display = 'none';
          document.getElementById("launch").style.display = 'none';
          document.getElementById("stop").style.display = 'none';
          document.getElementById("estop").style.display = 'block';

        }
        //alert("Pod Status: " + response);
      }).catch((err) => {
        alert("Error: " + err);
      });
    }

    return {setDestination, launch, stop, emergencyStop, getPodState}
  }
}
</script>
