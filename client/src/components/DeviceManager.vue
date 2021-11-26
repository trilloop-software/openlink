<template>
    <div>
        <h1>Connected Devices</h1>
        <ul id="dynamic-list">
          <li v-for="device in devices" :key="device.IP">
            <DeviceInterface/>
          </li>
        </ul>

        <button v-on:click="addItem()">add item</button>
        <button v-on:click="removeItem()">remove item</button>
    </div>

</template>

<script>
import DeviceInterface from './DeviceInterface.vue'

export default {
  name: 'DeviceManager',
  // define methods under the `methods` object
  methods: {
    addItem: function (){
        var ul = document.getElementById("dynamic-list");
        var li = document.createElement("DeviceInterface");
        //var li = document.createElement("li");
        //li.setAttribute('id','New Device');
        //li.appendChild(document.createTextNode('New Device'));
        ul.appendChild(li);
    },
    removeItem: function (){
        var ul = document.getElementById("dynamic-list");
        var item = document.getElementById('New Device');
        ul.removeChild(item);
    }
  },
  components: {
    DeviceInterface
  },
  data(){
    return{
      devices:
      [
        {IP:"0", name:"Placeholder Device"},
        {IP:"1", name:"Placeholder Device"},
        {IP:"2", name:"Placeholder Device"},
      ]
    }
  }
}

</script>
