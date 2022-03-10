<template>
  <q-page class="full-width column wrap" padding style="padding-top: 66px;">
    <q-page-sticky position="top" expand class="bg-grey text-white">
      <q-toolbar>
        <q-icon name="manage_accounts" size="md"/>
        <q-toolbar-title>Manage</q-toolbar-title>
      </q-toolbar>
    </q-page-sticky>

    <div class="fit row justify-end">
      <q-btn
        color="primary"
        flat
        dense
        icon-right="add_circle"
        label="ADD USER"
        @click="addUserDialog"
      />
    </div>

    <q-separator />

    <q-table
      class="q-mt-md"
      title="Users"
      :rows="userList"
      :columns="col"
      row-key="name"
      @row-click="editUser"
    />

    <user-add v-model:show="showAddDialog" :user="newUser" @add-user="addUser" />

    <user-edit 
      v-model:mode="editMode"
      v-model:show="showEditDialog"
      v-model:user="selectedUser"
      @remove-user="removeUser"
      @update-password="updateUserPassword"
      @update-usergroup="updateUserGroup"
    />

    <notification v-model:show="notifyShow" :kind="notifyKind" :msg="notifyMsg" />
  </q-page>
</template>

<style>
</style>

<script lang="ts">
import { ref, Ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import Notification from '@/components/Notification.vue'
import UserAdd from '@/components/UserAdd.vue'
import UserEdit from '@/components/UserEdit.vue'
import { getUserGroup, User, UserEditMode, UserSecure } from '@/libs/user'

export default {
  name: 'Manage',
  components: {
    Notification,
    UserAdd,
    UserEdit,
  },
  setup: () => {
    const userList: Ref<UserSecure[]> = ref([])
    const col = [
      {
        name: 'username',
        label: 'Username',
        align: 'left',
        field: 'name',
      },
      {
        name: 'usergroup',
        label: 'Usergroup',
        align: 'right',
        field: 'ugroup',
        format: (val) => getUserGroup(val),
      }
    ]
    const newUser = ref(new User)

    const notifyShow = ref(false)
    const notifyKind = ref('positive')
    const notifyMsg = ref('')
    const selectedUser = ref(new UserSecure)
    const showAddDialog = ref(false)
    const showEditDialog = ref(false)
    const editMode = ref(UserEditMode.Unselected)

    getUserList()

    function addUser(user: User) {
      const tempUser = new User
      tempUser.clone(user)

      invoke("add_user", { user: JSON.stringify(tempUser)})
        .then((response) => {
          userList.value.push(tempUser)
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
      
      showAddDialog.value = false
      newUser.value.clear()
    }

    function addUserDialog() {
      newUser.value.clear()
      showAddDialog.value = true
    }

    function editUser(evt, user) {
      selectedUser.value.clone(user)
      editMode.value = UserEditMode.Unselected
      showEditDialog.value = true
    }

    function getUserList() {
      invoke("get_user_list")
        .then((response) => {
          userList.value = JSON.parse(response as string)
        })
        .catch((error) => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
    }

    function removeUser(user: UserSecure) {
      invoke("remove_user", { name: user.name })
        .then(response => {
          userList.value.splice(userList.value.findIndex(el => el.name == user.name), 1)
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch(error => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

      showEditDialog.value = false
    }

    function updateUserGroup(user: UserSecure) {
      const tempUser = new UserSecure
      tempUser.clone(user)

      invoke("update_user_group", { user: JSON.stringify(user) })
        .then(response => {
          userList.value[userList.value.findIndex(el => el.name == tempUser.name)] = tempUser
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch(error => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })

      showEditDialog.value = false
      editMode.value = UserEditMode.Unselected
      selectedUser.value.clear()
    }

    function updateUserPassword(user: User) {
      invoke("update_user_password", { user: JSON.stringify(user) })
        .then(response => {
          notifyShow.value = true
          notifyKind.value = 'positive'
          notifyMsg.value = response as string
        })
        .catch(error => {
          notifyShow.value = true
          notifyKind.value = 'negative'
          notifyMsg.value = error as string
        })
      
      showEditDialog.value = false
      editMode.value = UserEditMode.Unselected
      selectedUser.value.clear()
    }

    return {
      addUser,
      addUserDialog,
      col,
      editMode,
      editUser,
      getUserList,
      newUser,
      notifyShow,
      notifyKind,
      notifyMsg,
      removeUser,
      selectedUser,
      showAddDialog,
      showEditDialog,
      updateUserGroup,
      updateUserPassword,
      userList,
    }
  }
}
</script>
