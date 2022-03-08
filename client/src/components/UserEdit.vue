<template>
  <q-dialog v-model="showDialog" persistent class="disable-select">
    <q-card>
      <q-bar>
        <q-item-label class="text-weight-bold">Modify User</q-item-label>
        <q-space />
        <q-btn flat icon="close" v-close-popup />
      </q-bar>

      <q-card-section class="fit column justify-center">
        <div v-if="activeMode == UserEditMode.Unselected" class="column">
          <q-btn
            color="primary"
            flat
            dense
            label="CHANGE PASSWORD"
            @click="activeMode = UserEditMode.Password"
          />
          <q-btn
            color="primary"
            flat
            dense
            label="CHANGE USERGROUP"
            @click="activeMode = UserEditMode.Usergroup"
          />
        </div>

        <q-input
          v-if="activeMode == UserEditMode.Password"
          dense
          filled
          v-model="password"
          :type="hidePassword ? 'password' : 'text'"
          label="Password"
        >
          <template v-slot:append>
            <q-icon
              :name="hidePassword ? 'visibility_off' : 'visibility'"
              class="cursor-pointer"
              @click="hidePassword = !hidePassword"
            />
          </template>
        </q-input>

        <q-select
          v-if="activeMode == UserEditMode.Usergroup"
          dense
          filled
          v-model.number="activeUser.ugroup"
          label="Usergroup"
          :options="userGroups"
          map-options
          emit-value
        />
      </q-card-section>

      <q-card-actions class="fit row justify-center">
        <q-btn
          v-if="activeMode == UserEditMode.Unselected"
          color="red"
          flat
          dense
          icon="delete"
          label="REMOVE"
          @click="removeUser(activeUser)"
        />

        <q-btn 
          v-if="activeMode == UserEditMode.Password"
          color="primary"
          flat
          dense
          icon-right="arrow_upward"
          label="UPDATE"
          @click="updatePassword(activeUser)"
        />

        <q-btn
          v-if="activeMode == UserEditMode.Usergroup"
          color="primary"
          flat
          dense
          icon-right="arrow_upward"
          label="UPDATE"
          @click="updateUsergroup(activeUser)"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { ref } from 'vue'
import { useModelWrapper } from '@/utils/modelWrapper'
import { User, UserEditMode, UserSecure } from '@/libs/user'

export default {
  name: 'UserEdit',
  props: {
    mode: { default: UserEditMode.Unselected },
    show: { type: Boolean, default: false },
    user: { type: UserSecure }
  },
  emits: ['update:mode','update:show','remove-user','update-password','update-usergroup'],
  setup: (props, { emit }) => {
    const hidePassword = ref(false)
    const password = ref('')
    const userGroups = [
      { label: 'Admin', value: 255 },
      { label: 'Software Team', value: 2 },
      { label: 'Mission Control', value: 1 },
    ]

    function removeUser(user: UserSecure) {
      emit('remove-user', user)
    }

    function updatePassword(userSecure: UserSecure) {
      const user = new User
      user.fromSecure(userSecure)
      user.pwd = password.value
      emit('update-password', user)
    }

    function updateUsergroup(user: UserSecure) {
      emit('update-usergroup', user)
    }

    return {
      activeMode: useModelWrapper(props, emit, 'mode'),
      activeUser: useModelWrapper(props, emit, 'user'),
      hidePassword,
      password,
      removeUser,
      showDialog: useModelWrapper(props, emit, 'show'),
      updatePassword,
      updateUsergroup,
      UserEditMode,
      userGroups,
    }
  }
}
</script>
