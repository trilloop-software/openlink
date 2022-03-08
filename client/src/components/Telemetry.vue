<template>
  <h2>Telemetry Data</h2>
  <v-simple-table id = "telemetry" width="100%">
      <thead>
        <tr>
          <th class="text-left">
            Category
          </th>
          <th class="text-left">
            Value
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="item in data"
          :key="item.value"
        >
          <td>{{ item.category }}</td>
          <td>{{ item.value }}</td>
        </tr>
      </tbody>
  </v-simple-table>
</template>

<style>

#telemetry {
  font-family: Arial, Helvetica, sans-serif;
  border-collapse: collapse;
  width: 100%;
  table-layout: fixed;
  display: table;
}

#telemetry td, #telemetry th {
  border: 1px solid #ddd;
  padding: 8px;
}

#telemetry tr:nth-child(even){background-color: #f2f2f2;}

#telemetry tr:hover {background-color: #ddd;}

#telemetry th {
  padding-top: 12px;
  padding-bottom: 12px;
  text-align: left;
  background-color: #4F2683;
  color: white;
}

</style>

<script>

export default {
    data() {
      return {
        data: [
          {category: 'Accelerometer', value: 0},
          {category: 'Brake Temperature', value: 50},
          {category: 'Battery Temperature', value: 30},
          {category: "Battery Current", value: 12}
        ]
    }
  }
}

</script>
