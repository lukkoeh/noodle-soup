<template>
  <button 
    :class="buttonClasses"
    :disabled="disabled"
    @click="handleClick"
  >
    <slot>{{ text }}</slot>
  </button>
</template>

<script>
export default {
  name: 'SimpleButton',
  props: {
    text: {
      type: String,
      default: 'Button'
    },
    variant: {
      type: String,
      default: 'primary',
      validator: value => ['primary', 'secondary', 'accent', 'outline'].includes(value)
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
  },
  computed: {
    buttonClasses() {
      const base = "inline-flex items-center justify-center border font-medium rounded-lg cursor-pointer transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 active:translate-y-px"
      
      const variants = {
        primary: "bg-widget text-norm border-misc hover:bg-input hover:border-accent focus:ring-accent-t",
        secondary: "bg-input text-norm border-misc hover:bg-misc focus:ring-accent-t",
        accent: "bg-accent text-white border-accent hover:bg-accent-l hover:border-accent-l focus:ring-accent-t",
        outline: "bg-transparent text-accent border-accent hover:bg-accent-t focus:ring-accent-t"
      }
      
      const sizes = {
        small: "px-3 py-1.5 text-xs min-h-7",
        medium: "px-4 py-2 text-sm min-h-9",
        large: "px-6 py-3 text-base min-h-11"
      }
      
      const disabled = this.disabled ? "opacity-50 cursor-not-allowed pointer-events-none" : ""
      
      return `${base} ${variants[this.variant]} ${sizes[this.size]} ${disabled}`.trim()
    }
  },
  methods: {
    handleClick(event) {
      if (!this.disabled) {
        this.$emit('click', event);
      }
    }
  }
}
</script>