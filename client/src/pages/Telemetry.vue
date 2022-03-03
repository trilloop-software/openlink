<template>
  <q-page class="full-width column wrap" padding style="padding-top: 66px;">
    <q-page-sticky position="top" expand class="bg-grey text-white">
      <q-toolbar>
        <q-icon name="settings" size="md"/>
        <q-toolbar-title>Telemetry</q-toolbar-title>
      </q-toolbar>
    </q-page-sticky> 

<v-simple-table>
    <template v-slot:default>
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
          v-for="item in items"
          :key="item.value"
        >
          <td>{{ item.category }}</td>
          <td>{{ item.value }}</td>
        </tr>
      </tbody>
    </template>
  </v-simple-table>

  </q-page>
</template>

<script>

export default {
    data() {
      return {
        items: [
          {category: 'Accelerometer', value: 0},
          {category: 'Brake Temperature', value: 50},
          {category: 'Battery Temperature', value: 30},
          {category: "Battery Current", value: 12}
        ]
    }
  }
}

</script>
