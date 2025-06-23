<script setup>
import { computed } from 'vue';

const props = defineProps({
  iconStyle: {
    type: String,
    default: "fa-regular",
  },
  icon: {
    type: String,
    required: true,
  },
  class: {
    type: String,
    required: false,
  },
})

const classes = computed(() => [props.iconStyle, props.icon, props.class].join(" "))
</script>

<template>
  <i :class="classes"></i>
</template>