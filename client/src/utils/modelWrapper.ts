// clean and simple v-model two-way binding
// from https://vanoneang.github.io/article/v-model-in-vue3.html
import { computed } from 'vue'
export function useModelWrapper(props, emit, name = 'modelValue') {
  return computed({
    get: () => props[name],
    set: (value) => emit(`update:${name}`, value)
  })
}
