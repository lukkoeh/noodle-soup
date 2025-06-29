<script setup>
import { computed } from 'vue';

const props = defineProps({
  iconStyle: {
    type: String,
    default: "fa-regular",
  },
  icon: {
    type: String,
    required: false,
  },
  class: {
    type: String,
    required: false,
  },
  type: {
    type: String,
    required:false
  }
})

const classes = computed(() => {
  let classes = [props.iconStyle, props.icon, props.class].join(" ");

  switch(props.type){
    case "plus":
      classes = "fa-plus fa-solid"
      break;
    case "link":
      classes = "fa-solid fa-link"
      break;
    case "markdown":
      classes = "fa-brands fa-markdown"
      break;
    case "images":
      classes = "fa-solid fa-images"
      break;
    case "question":
      classes = "fa-solid fa-question"
      break;
  }
  return classes

});



</script>

<template>
  <i :class="classes"></i>
</template>