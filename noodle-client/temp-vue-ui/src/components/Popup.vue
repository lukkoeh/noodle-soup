<script setup>
  defineProps({
    isOpen: {
      type: Boolean,
      default: false
    },
    title: {
      type: String,
      default: 'Popup Titel'
    },
    closeOnBackdrop: {
      type: Boolean,
      default: true
    }
  })
  const emit = defineEmits(["close"]);

  function closePopup(){
    emit('close')
  }
</script>

<template>
  <!-- Backdrop - Abdunklung des gesamten Screens -->
  <div
    v-if="isOpen"
    class="fixed inset-0 bg-black/40 flex items-center justify-center z-50"
    @click="closeOnBackdrop && closePopup()"
  >
    <!-- Popup Container -->
    <div
      class="bg-widget rounded-lg shadow-xl max-w-md w-full mx-4 max-h-[90vh] overflow-hidden"
      @click.stop
    >
      <!-- Header -->
      <div class="flex items-center justify-between p-4">
        <h2 class="text-lg font-semibold text-normal">
          {{ title }}
        </h2>
        <button
          @click="closePopup"
          class="text-normal hover:text-gray-600 transition-colors p-1 rounded-full hover:bg-gray-100"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>

      <!-- Content Area -->
      <div class="p-4 overflow-y-auto">
        <slot></slot>
      </div>
    </div>
  </div>
</template>