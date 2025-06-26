
<script setup>
import { ref, computed } from 'vue'
import Header from '@/components/Header.vue'
import { useRoute } from 'vue-router'
import CourseElement from '@/components/CourseElement.vue'
import Button from '@/components/Button.vue'
import Popup from '@/components/Popup.vue'

// enthält die ID des aktiven Kurses aus der URL
const route = useRoute()
const courseId = route.params.id

// Reactive data
const editMode = ref(true)
const showAddElementMenu = ref(false)
const Sections = ref([
  {
    "sectionId": 1,
    "parentCourseId": 123,
    "headline": "Section 1",
    "content": [{
      "uid": 456,
      "courseId": 123,
      "parentSectionId": 1,
      "type": "markdown",
      "content": "# This is Markdown\n- I am a list Item\n- mee too",
      "files": [
        {
          "targetUid": 789
        }
      ]
    },
    {
      "uid": 457,
      "courseId": 123,
      "parentSectionId": 1,
      "type": "markdown",
      "content": "I am some Text",
      "files": [
        {
          "targetUid": 789
        }
      ]
    },]
  },
  {
    "sectionId": 2,
    "parentCourseId": 123,
    "headline": "Section 2", 
    "content": [{
      "uid": 456,
      "courseId": 123,
      "parentSectionId": 1,
      "type": "markdown",
      "content": "base64encodedcontent",
      "files": [
        {
          "targetUid": 789
        }
      ]
    }]
  }
])

const availableElements = ref([
  { type: 'markdown', label: 'Markdown' },
  { type: 'link', label: 'Verlinkungselement' },
  { type: 'media', label: 'Media Element' },
  { type: 'quiz', label: 'Quiz Element' }
])

const duplicateElement = (type) => {
  console.log(`Duplicating ${type} element`)
}

const addElement = (type) => {
  console.log(`Adding ${type} element`)
  showAddMenu.value = false
}
const addSection = () => {

}
const handleAddElement = () => {
  showAddElementMenu.value = false
}
const handleElementUpdate = () => {

}
</script>

<template>
  <Header/>
  <div class="flex h-max bg-main">
    <!-- Header -->
    <div class="fixed top-0 left-0 right-0 h-1 bg-white border-b border-gray-200 flex items-center justify-between px-5 z-50">
      <span class="font-semibold text-base">{{ courseTitle }}</span>
    </div>
    <!-- Inhalts übersicht -->
    <div class="w-64 bg-white border-r border-gray-200 pt-16 pb-5 overflow-y-auto">
      <div
      v-for="section in Sections"
      >
        <h2
        class="text-xl"
        >{{ section.headline }}</h2>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex flex-col pt-16 px-5 pb-5 gap-6 overflow-y-auto">
      <div
      class="flex flex-col gap-4"
      v-for="section in Sections">
        <h1
        class="font-bold text-2xl"
        >{{ section.headline }}</h1>
        <CourseElement
        v-for="element in section.content"
        :element="element"
        :editMode="editMode"
        @update="() => handleElementUpdate(element.uid)"
        />
      </div>
      <div
      v-if="editMode == true"
      class="flex gap-6 justify-center"
      >
        <Button
        @click="showAddElementMenu = true"
        >Add Element</Button>
        <Button
        @click=""
        >Add Section</Button>
        <Popup
        :is-open="showAddElementMenu"
        @close="showAddElementMenu = false"
        title="Element hinzufügen"
        >
          <Button
          v-for="element in availableElements"
          @click="() => handleAddElement(element.type)"
          >{{ element.label }}</Button>
        </Popup>
      </div>
    </div>
  </div>
</template>
