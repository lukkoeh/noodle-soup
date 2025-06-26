<script setup>
import { ref, computed, onMounted } from 'vue';
import UserList from '@/components/UserList.vue';
import Header from '@/components/Header.vue';
import Popup from '@/components/Popup.vue';
import Icon from '@/components/Icon.vue';
import LineInput from '@/components/LineInput.vue';
import Button from '@/components/Button.vue';
import AddUser from '@/components/AddUser.vue';
import AddCourse from '@/components/AddCourse.vue';

// Reactive data
const showAddCourseModal = ref(false)
const showAddUserModal = ref(false)
const selectedCourse  = ref({name:""})
const newCourse = ref({})

// Sample data - kurse
const courses = ref([
  {
    uid: 123,
    "name": "Course 1",
    "content": ['...']
  },
  {
    uid: 223,
    "name": "Course 2",
    "content": ['...']
  },
])

// Sample data - Users (wird normalerweise basierend auf ausgewählter Gruppe geladen)
const courseUsers = ref([
  {
    selected: false,
    userId: 10,
    firstname: 'Maja',
    lastname: 'Biene',
    email: 'm.b@mail.de',
    title: 'Studiengangsleiter'
  }
])

const allUsers = ref([
    {
    selected: false,
    userId: 11,
    firstname: 'Mana',
    lastname: 'Biene',
    email: 'm.b@mail.de',
    title: 'Student'
  },
  {
    selected: false,
    userId: 13,
    firstname: 'Karl',
    lastname: 'Biene',
    email: 'm.b@mail.de',
    title: 'Student'
  }
])

const selectCourse = (course)=>{
    selectedCourse.value = course;
}

const addUserToCourse = () =>{

}

</script>

<template>
    <div class="">
    <Header/>
    <div class="flex justify-between my-6">
        <div
        class="flex gap-6 mb-6">
            <h1
            class="text-2xl"
            >Alle Kurse</h1>
            <Button
            @click="showAddCourseModal = true"
            >
                <Icon
                icon="fa-plus"
                icon-style="fa-solid"
                />
            </Button>
        </div>
        <div>
            <LineInput
            placeholder="Kurs suchen"
            intype="search"/>
        </div>
    </div>
    
    <div class="flex bg-main items-stretch px-4">
    <div
    class="bg-main flex flex-col gap-2 justify-start items-end border-r-3 border-misc"
    >
        <p
        v-for="course in courses"
        @click="()=>selectCourse(course)"
        :class="['rounded-l-full px-4', selectedCourse.uid == course.uid ? 'bg-input' : 'border-1 border-r-0 border-misc bg-main']"
        >
            {{ course.name }}
        </p>
    </div>
    <!--Course edit section-->
    <div class="flex flex-col justify-between grow-1 gap-6 px-4">
        <div class="flex gap-6 justify-between">
            <div>
                <h2 class="text-xl">Titel</h2>
                <LineInput
                v-model="selectedCourse.name"
                placeholder="Kurs Name"
                />
            </div>
            <div>
                <h2 class="text-xl">Kürzel</h2>
                <LineInput
                v-model="selectedCourse.name"
                placeholder="Kürzel"
                />
            </div>
        </div>
        <!-- User Select section-->
        <div class="flex flex-col gap-6">
            <!--Add User-->
            <div>
                <h2 class="text-2xl">
                    User
                    <Button
                    @click="showAddUserModal = true"
                    type="primary">
                    <Icon icon="fa-plus" icon-style="fa-solid"/>
                    </Button>
                </h2>

            </div>
            <UserList
            :users="courseUsers"
            />

        </div>
        <div class="flex justify-end gap-6">
            <Button
            type="secondary"
            >
                Kurs Löschen
            </Button>
            <Button>
                speichern
            </Button>
        </div>
    </div>
    <!--collect all popups here-->
    <Popup title="User hinzufügen" :is-open="showAddUserModal" @close="showAddUserModal = false">
      <AddUser
      v-model="allUsers"
      @add-user="(data)=>addUserToCourse(data)"
      />
    </Popup>
    <Popup
    title="Neuen Kurs anlegen"
    :is-open="showAddCourseModal"
    @close="showAddCourseModal = false"
    >
        <AddCourse
        v-model="newCourse"
        v-model:users="allUsers"
        />
    </Popup>
  </div>
  </div>
</template>
