
<script setup>
import { ref, computed, onMounted } from 'vue'
import ToggleInput from '@/components/ToggleInput.vue'
import { useRoute } from 'vue-router'
import CourseElement from '@/components/CourseElement.vue'
import { availableElements } from '@/components/CourseElement.vue'
import Button from '@/components/Button.vue'
import Icon from '@/components/Icon.vue'
import { fetchContentForSection, fetchCourse, fetchSectionsForCourse } from '@/utils/api'
import LineInput from '@/components/LineInput.vue'

// enthält die ID des aktiven Kurses aus der URL
const route = useRoute()
const courseId = route.params.id
const courseTitle = ref("Kurs name")

// Reactive data
const editMode = ref(false)
const hasEditPermission = ref(true)

const showAddElementMenu = ref(false)


const Sections = ref([/*
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
          "targetUid": 789sectionId
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
  }*/
])

onMounted(async () => {
  const rco = await fetchCourse(courseId)
  if (rco.status === 401)
    window.location.href = "/login"
  if (rco.status === 200)
    courseTitle.value = rco.body.name

  const rcs = await fetchSectionsForCourse(courseId)
  //TODO: put these into one request eventually
  let sections = []
  if (rcs.status === 200)
    sections = rcs.body

  for(let s of sections) {
    const rc = await fetchContentForSection(courseId, s.sectionId)
    if (rc.status === 200)
      s.content = rc.body
  }

  Sections.value = sections
})


const handleAddSection = (addAtIndex) => {
  //Todo get new sectionId from server
  const emptySection = {
  "sectionId": 1,
  "parentCourseId": courseId,
  "headline": "New Section",
  "content": [],
  }


  const newSections = [...Sections.value.slice(0, addAtIndex), emptySection, ...Sections.value.slice(addAtIndex)]; 
  console.log('NewSection: ', newSections)
  Sections.value = newSections;
}
const handleAddElement = (elementType, addAtIndex, sectIndex, parentSectionId) => {
  //Todo getnew uid from server
  const newElement = {
      "courseId": courseId,
      "parentSectionId": parentSectionId,
      "type": elementType,
      "content": "",
  };
  console.log('section', Sections.value[sectIndex], 'sectionindex: ', sectIndex)

  const newElements = [...Sections.value[sectIndex].content.slice(0, addAtIndex), newElement, ...Sections.value[sectIndex].content.slice(addAtIndex)]

  Sections.value[sectIndex].content = newElements;
}
const handleElementUpdate = () => {

}
const handleSave = () => {

}
</script>

<template>
  <div class="flex h-full bg-main grow">
    
    <!-- Inhalts übersicht -->
    <div class="w-64 h-full border-input dark:border-widget border-r-2 pb-5 overflow-y-auto al">
      <h2
      v-if="!editMode"
      class="pb-4 text-2xl"
      >{{ courseTitle }}</h2>
      <div
      v-for="section in Sections"
      >
        <h2
        class="text-xl"
        >{{ section.headline }}</h2>
      </div>
    </div>

    <!-- Main Content -->
    <div class="h-1 min-h-full flex-grow overflow-y-scroll">
      <div class="flex flex-col px-4 py-2 gap-6  grow">
        <!-- Header -->
        <div class=" bg-main border-b border-misc sticky top-0 flex justify-between p-2">
          <span class="font-semibold  text-4xl">
            <span
            v-if="!editMode">
            {{ courseTitle }}</span>
            <LineInput
            v-else
            v-model="courseTitle"
            />
            <ToggleInput
            v-if="hasEditPermission"
            v-model="editMode"
            :icon="['fa-pencil']"
            :icon-style="['fa-solid']"
            />
          </span>
          <Button
          v-if="editMode == true"
          @click="handleSave">
          Änderungen Speichern</Button>
        </div>
        <div
          v-if="(editMode == true)"
          class="relative h-6 flex items-center">
            <hr class="border-accent grow"/>
            <Button
            type="secondary"
            size="small"
            class="absolute left-1/2 -translate-x-1/2 bg-widget"
            @click="() => handleAddSection(0)"
            ><Icon type="plus"></Icon> Add Section</Button>
        </div>
        <div
        class="flex flex-col gap-6  h-max"
        v-for="(section, sectionIndex) in Sections">
        <div
        class="flex flex-col gap-6 bg-input dark:bg-widget p-6 rounded-3xl h-max">
          <h2
          class="font-bold text-2xl"
          >
            <span v-if="!editMode">{{ section.headline }}</span>
            <LineInput
            class="border-accent border"
            v-if="editMode"
            v-model="Sections[sectionIndex].headline"
            />
          </h2>
          <div
            v-if="(editMode == true)"
            class="relative h-6 flex items-center">
            <hr class="border-accent grow"/>
            <div
            class="absolute left-1/2 -translate-x-1/2 flex gap-4">
                <Button
              v-for="addElement in availableElements"
              type="secondary"
              class="bg-widget"
              @click="() => handleAddElement(addElement.type, 0, sectionIndex, section.sectionId)"
              ><Icon :type="addElement.icon"></Icon> {{ addElement.label }}</Button>
            </div>
          </div>
          <CourseElement
          v-for="(element, elemIndex) in section.content"
          :element="element"
          :editMode="editMode"
          @update="() => handleElementUpdate(element.uid)"
          >
            <div
            v-if="editMode == true"
            class="relative h-6 flex items-center">
            <hr class="border-accent grow"/>
            <div
            class="absolute left-1/2 -translate-x-1/2 flex gap-4">
                <Button
              v-for="addElement in availableElements"
              type="secondary"
              class="bg-widget"
              @click="() => handleAddElement(addElement.type, (elemIndex + 1), sectionIndex, section.sectionId)"
              ><Icon :type="addElement.icon"></Icon> {{ addElement.label }}</Button>
            </div>
          </div>
          </CourseElement>
        </div>
          
          
          <div
          v-if="editMode == true"
          class="relative h-6 flex items-center">
            <hr class="border-accent grow"/>
            <Button
            type="secondary"
            size="small"
            class="absolute left-1/2 -translate-x-1/2 bg-widget"
            @click="() => handleAddSection(sectionIndex + 1)"
            ><Icon type="plus"></Icon> Add Section</Button>
          </div>
        </div>
        
      </div>
     </div>
    
  </div>
</template>
