<template>
  <button 
    :class="['btn', `btn--${variant}`, `btn--${size}`, { 'btn--disabled': disabled }]"
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
  methods: {
    handleClick(event) {
      if (!this.disabled) {
        this.$emit('click', event);
      }
    }
  }
}
</script>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 8px;
  font-family: inherit;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  text-decoration: none;
  outline: none;
  position: relative;
  overflow: hidden;
}

.btn:focus {
  box-shadow: 0 0 0 2px var(--color-accent-t);
}

.btn:active {
  transform: translateY(1px);
}

/* Sizes */
.btn--small {
  padding: 6px 12px;
  font-size: 12px;
  min-height: 28px;
}

.btn--medium {
  padding: 8px 16px;
  font-size: 14px;
  min-height: 36px;
}

.btn--large {
  padding: 12px 24px;
  font-size: 16px;
  min-height: 44px;
}

/* Variants */
.btn--primary {
  background-color: var(--color-widget);
  color: var(--color-norm);
  border: 1px solid var(--color-misc);
}

.btn--primary:hover:not(.btn--disabled) {
  background-color: var(--color-input);
  border-color: var(--color-accent);
}

.btn--secondary {
  background-color: var(--color-input);
  color: var(--color-norm);
  border: 1px solid var(--color-misc);
}

.btn--secondary:hover:not(.btn--disabled) {
  background-color: var(--color-misc);
}

.btn--accent {
  background-color: var(--color-accent);
  color: white;
  border: 1px solid var(--color-accent);
}

.btn--accent:hover:not(.btn--disabled) {
  background-color: var(--color-accent-l);
  border-color: var(--color-accent-l);
}

.btn--outline {
  background-color: transparent;
  color: var(--color-accent);
  border: 1px solid var(--color-accent);
}

.btn--outline:hover:not(.btn--disabled) {
  background-color: var(--color-accent-t);
}

/* Disabled state */
.btn--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.btn--disabled:hover {
  transform: none;
}
</style>