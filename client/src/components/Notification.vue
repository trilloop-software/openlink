<template>
  <div class="q-pa-md q-gutter-sm">
    <q-dialog v-model="notifyShow" seamless position="bottom">
      <q-card style="width: 350px" :class="`bg-${kind} text-white`">
        <q-card-section class="row items-center no-wrap text-center">
          <q-icon :name="kind == 'positive' ? 'check_circle' : kind == 'negative' ? 'error' :  kind == 'info' ? 'info' : 'warning'" style="font-size: 2em;" />

          <q-space />

          <div>
            {{ msg }}
          </div>

          <q-space />

          <q-btn flat round icon="close" v-close-popup />
        </q-card-section>
      </q-card>
    </q-dialog>
  </div>
</template>

<script lang="ts">
import { useModelWrapper } from '@/utils/modelWrapper'

export default {
  name: 'Notification',
  props: {
    show: { type: Boolean, default: false },
    kind: { type: String, default: 'positive' }, // 'positive', 'negative', 'info', 'warning' (built in quasar colours)
    msg: { type: String, default: '' }
  },
  setup: (props: any, { emit }) => {
    return {
      notifyShow: useModelWrapper(props, emit, 'show'),
    }
  }
}

</script>
