<template>
    <div>
        <h1>Connected Devices</h1>
        <ul id="dynamic-list"></ul>

        <button v-on:click="addItem()">add item</button>
        <button v-on:click="removeItem()">remove item</button>
    </div>

</template>

<script>
export default {
  name: 'DeviceManager',
  // define methods under the `methods` object
  methods: {
    addItem: function (){
        var ul = document.getElementById("dynamic-list");
        var li = document.createElement("li");
        li.setAttribute('id','New Device');
        li.appendChild(document.createTextNode('New Device'));
        ul.appendChild(li);
    },
    removeItem: function (){
        var ul = document.getElementById("dynamic-list");
        var item = document.getElementById('New Device');
        ul.removeChild(item);
    }
  }
}

</script>
