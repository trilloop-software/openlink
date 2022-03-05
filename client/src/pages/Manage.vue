<template>
  <q-page class="flex flex-center">
    Manage
    <br />
    <q-btn push label="Create" @click="addUser" />
    <q-btn push label="List" @click="getUserList" />
    <q-btn push label="Remove" @click="removeUser" />
    <q-btn push label="Group" @click="updateUserGroup" />
    <q-btn push label="Pwd" @click="updateUserPassword" />
  </q-page>
</template>

<style>
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

export default {
  name: 'Manage',
  setup: () => {
    function addUser() {
      invoke("add_user", { user: JSON.stringify({ name: 'test', pwd: 'test', ugroup: 1 })})
        .then((response) => {
          alert(response)
        })
        .catch((error) => {
          alert(error)
        })
    }

    function getUserList() {
      invoke("get_user_list")
        .then((response) => {
          alert(response)
        })
        .catch((error) => {
          alert(error)
        })
    }

    function removeUser() {
      invoke("remove_user", { name: "test" })
        .then(response => {
          alert(response)
        })
        .catch(error => {
          alert(error)
        })
    }

    function updateUserGroup() {
      invoke("update_user_group", { user: JSON.stringify({ name: "admin", ugroup: 3 }) })
        .then(response => {
          alert(response)
        })
        .catch(error => {
          alert(error)
        })
    }

    function updateUserPassword() {
      invoke("update_user_password", { user: JSON.stringify({ name: "test", pwd: "success", ugroup: 2 }) })
        .then(response => {
          alert(response)
        })
        .catch(error => {
          alert(error)
        })
    }

    return {
      addUser,
      getUserList,
      removeUser,
      updateUserGroup,
      updateUserPassword,
    }
  }
}
</script>
