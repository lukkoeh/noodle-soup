<script setup>
import { ref } from 'vue'

const searchQuery = ref('')
</script>

<template>
  <header class="bg-widget px-4 py-3 shadow-sm w-full">
    <div class="flex items-center justify-between max-w-7xl mx-auto">
      <!-- Logo Section -->
      <div class="bg-gray-300 px-6 py-2 rounded">
        <span class="text-gray-700 font-medium text-sm">LOGO</span>
      </div>

      <!-- Search and Icons Section -->
    </div>
    <div class="flex items-center space-x-4">
      <!-- Search Input -->
      <div class="relative">
        <input v-model="searchQuery" type="text" placeholder="Suchen"
          class="bg-white border border-gray-300 rounded px-3 py-1.5 pr-8 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-64" />
        <button class="absolute right-2 top-1/2 transform -translate-y-1/2 text-gray-500 hover:text-gray-700">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
          </svg>
        </button>
      </div>

      <!-- Shopping Cart Icon -->
      <button class="text-gray-600 hover:text-gray-800 p-1">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-1.1 5.4M7 13v8a2 2 0 01-2 2H4m15-10v8a2 2 0 01-2 2h-1M9 19v2m4-2v2">
          </path>
        </svg>
      </button>

      <!-- User Profile Icon -->
      <button class="text-gray-600 hover:text-gray-800 p-1">
        <div class="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center">
          <svg class="w-4 h-4 text-gray-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd">
            </path>
          </svg>
        </div>
      </button>
    </div>
  </header>
</template>