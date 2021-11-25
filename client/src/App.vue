<template>
  <div>
    <!--<img alt="Vue logo" src="./assets/logo.png">-->
    <!--<HelloWorld msg="Welcome to Your Vue.js App"/>-->
    <DeviceManager/>
  </div>
</template>


<script>
//import HelloWorld from './components/HelloWorld.vue'
import DeviceManager from './components/DeviceManager.vue'

export default {
  name: 'App',
  components: {
    //HelloWorld,
    DeviceManager
  }
}

</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
