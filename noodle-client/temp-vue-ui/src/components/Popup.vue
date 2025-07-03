<script setup>
import Icon from './Icon.vue';

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
      default: false
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
      class="bg-widget rounded-lg shadow-xl max-w-[75vw] min-w-[30vw] mx-4 max-h-[90vh] flex flex-col"
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
          <Icon icon="fa-x" icon-style="fa-solid"></Icon>
        </button>
      </div>

      <!-- Content Area -->
      <div class="p-4">
        <slot></slot>
      </div>
    </div>
  </div>
</template>