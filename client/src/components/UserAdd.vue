<template>
  <q-dialog v-model="showDialog" persistent class="disable-select">
    <q-card>
      <q-bar>
        <q-item-label class="text-weight-bold">Add User</q-item-label>
        <q-space />
        <q-btn flat icon="close" v-close-popup />
      </q-bar>

      <q-card-section class="q-gutter-y-md">
        <q-input
          dense
          filled
          v-model="newUser.name"
          type="text"
          label="Name"
        />
        <q-input
          dense
          filled
          v-model="newUser.pwd"
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
          dense
          filled
          v-model.number="newUser.ugroup"
          label="Usergroup"
          :options="userGroups"
          map-options
          emit-value
        />
      </q-card-section>

      <q-separator />

      <q-btn 
        class="fit row content-end"
        flat
        color="primary"
        label="ADD"
        icon-right="add"
        @click="addNewUser(newUser)"
      />
    </q-card>
  </q-dialog>

</template>

<script lang="ts">
import { ref, PropType } from 'vue'
import { useModelWrapper } from '@/utils/modelWrapper'
import { User } from "@/libs/user"

export default {
  name: 'UserAdd',
  props: {
    show: { type: Boolean, default: false },
    user: { type: Object as PropType<User>, default: new User }
  },
  emits: ['update:show','update:user','add-user'],
  setup: (props: any, { emit }) => {
    const hidePassword = ref(true)
    const userGroups = [
      { label: 'Admin', value: 255 },
      { label: 'Software Team', value: 2 },
      { label: 'Mission Control', value: 1 },
    ]

    function addNewUser(user: User) {
      console.log(user)
      emit('add-user', user)
    }

    return {
      addNewUser,
      hidePassword,
      newUser: useModelWrapper(props, emit, 'user'),
      showDialog: useModelWrapper(props, emit, 'show'),
      userGroups,
    }
  }
}
</script>
