<script setup>
import { ref } from 'vue';
import TagBubbles from './TagBubbles.vue';
import ToggleInput from './ToggleInput.vue';

const emit = defineEmits(["bookmarked"])

const props = defineProps({
  course:{
    type: Object,
    required: true,
    default: {
      tags: ['one', 'two'],
      dozenten: ['Prof. Dr. Arnulf Mester'],
      description: "T7 | Web Engineering und ganz viel Weiteres",
      bookmarked: false,
    }
  },
})

function updateBookmark(newValue){

}
</script>

<template>
  <div class="w-80 bg-widget rounded-3xl shadow-lg relative">

    <div class="rounded-t-3xl h-20 w-full bg-accent-t"></div>
    <span class="absolute top-16 right-8 text-3xl">
        <ToggleInput
          v-model="course.bookmarked"
          :icon="['fa-bookmark', 'fa-bookmark']"
          :icon-style="['fa-solid', 'fa-regular']"
          @toggled="updateBookmark"
        />
    </span>
    <div class="p-5 w-full">
      
      <!-- Tags -->
      <TagBubbles :tag-list="course.tags"/>
      
      <!-- Content -->
      <h2 class="text-lg font-bold text-norm mb-2 leading-tight">{{ props.course.description }}</h2>
      <p v-for="dozent in props.course.dozent" class="text-sm text-norm italic">{{ dozent }}</p>
    </div>
    
  </div>
</template>