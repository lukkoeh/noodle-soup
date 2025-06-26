<script setup>
import { ref } from 'vue';
import TagBubbles from './TagBubbles.vue';
import ToggleInput from './ToggleInput.vue';
import { useRouter } from 'vue-router'

const router = useRouter()
const emit = defineEmits(["bookmarked"])

const props = defineProps({
  course:{
    type: Object,
    required: true,
    default: {
      uid: 123,
      tags: ['one', 'two'],
      dozenten: ['Prof. Dr. Arnulf Mester'],
      description: "T7 | Web Engineering und ganz viel Weiteres",
      bookmarked: false,
    }
  },
})

function updateBookmark(newValue){

}

function goToCourse(event, id) {
  event.preventDefault();
  router.push(`/course/${id}`)
}

</script>

<template>
  <div
  class="w-80 bg-widget rounded-3xl shadow-lg relative flex-shrink-0"
  @click="(event)=>goToCourse(event, props.course.uid)"
  >

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