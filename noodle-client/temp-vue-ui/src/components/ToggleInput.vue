<script setup>
  import { computed } from 'vue';
  import Icon from './Icon.vue';

  const props = defineProps({
    icon:{
      type: Array,
      default: ['fa-square-check', 'fa-square'],
    },
    iconStyle: {
      type: Array,
      default: ['fa-regular'],
    },
    label: {
      type: String,
      default: '',
    },
  })

  const emit = defineEmits(['toggled'])

  const model = defineModel()

  const classes = computed(() => !model.value ? "text-light" : "text-accent");
  const styleEval = computed(() => !model.value && (1 in props.iconStyle)? props.iconStyle[1] :  props.iconStyle[0]);
  const iconEval = computed(() => !model.value && (1 in props.icon) ? props.icon[1] : props.icon[0]);

  function toggled(event){
    model.value = !model.value;
    emit('toggled', event);
  }
</script>

<template>
  <label :class="classes">
    <input
      type="checkbox"
      :checked="model.value"
      @change="toggled"
      class="hidden"
    />
    <Icon
      :icon="iconEval"
      :icon-style="styleEval"
    />
    <label class="label-text" v-if="label">{{ label }}</label>
  </label>
</template>
