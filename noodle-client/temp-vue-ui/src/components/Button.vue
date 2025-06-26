<template>
  <button 
    :class="buttonClasses"
    :disabled="disabled"
  >
    <slot>{{ text }}</slot>
  </button>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  text: {
    type: String,
    default: 'Button'
  },
  type: {
    type: String,
    default: 'primary',
    validator: value => ['primary', 'secondary', 'accent', 'outline', 'simple'].includes(value)
  },
  size: {
    type: String,
    default: 'medium',
    validator: value => ['small', 'medium', 'large'].includes(value)
  },
  disabled: {
    type: Boolean,
    default: false
  }
})

const buttonClasses = computed(() => {
  const base = "inline-flex items-center justify-center font-medium rounded-full cursor-pointer transition-all duration-200 focus:outline-none focus:ring-2 active:translate-y-px"
  
  const types = {
    primary: "bg-accent text-norm hover:bg-input hover:border-accent-l focus:ring-accent-t",
    secondary: "bg-transparent text-accent border border-accent hover:bg-misc focus:ring-accent-t",
    accent: "bg-accent text-norm border-accent hover:bg-accent-l hover:border-accent-l focus:ring-accent-t",
    outline: "bg-transparent text-accent border-accent hover:bg-accent-t focus:ring-accent-t",
    simple: "bg-transparent hover:bg-input h-9 w-9"
  }
  
  const sizes = {
    small: "px-3 py-1.5 text-xs min-h-7",
    medium: "px-4 py-2 text-sm min-h-9",
    large: "px-6 py-3 text-base min-h-11"
  }
  
  const disabled = props.disabled ? "opacity-50 cursor-not-allowed pointer-events-none" : ""
  
  return `${base} ${types[props.type]} ${sizes[props.size]} ${disabled}`.trim()
})
</script>