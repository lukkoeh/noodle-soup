<script setup>
import { ref } from 'vue';
import TagBubbles from './TagBubbles.vue';
import ToggleInput from './ToggleInput.vue';

const tags = ref(['one', 'two'])
const dozenten = ref(['Prof. Dr. Arnulf Mester'])
const description = ref("T7 | Web Engineering und ganz viel Weiteres")
</script>

<template>
  <div class="w-80 bg-widget rounded-3xl shadow-lg relative">

    <div class="rounded-t-3xl h-20 w-full bg-accent-t"></div>
    <div class="p-5 w-full">
      <!-- Header -->
      <div class="flex justify-end items-center mb-4 relative">
        <div class=" absolute bottom-0 w-0 h-0 border-l-[12px] border-r-[12px] border-b-[15px] border-l-red-500 border-r-red-500 border-b-transparent border-t-[15px] border-t-red-500"></div>
        
      </div>
      
      <!-- Tags -->
      <TagBubbles :tag-list="tags"/>
      
      <!-- Content -->
      <h2 class="text-lg font-bold text-norm mb-2 leading-tight">{{ description }}</h2>
      <p v-for="dozent in dozenten" class="text-sm text-norm italic">{{ dozent }}</p>
    </div>
    
  </div>
</template>