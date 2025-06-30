<script setup>
import { ref, computed, onMounted } from 'vue';
import UserList from '@/components/UserList.vue';
import Popup from '@/components/Popup.vue';
import Icon from '@/components/Icon.vue';
import LineInput from '@/components/LineInput.vue';
import Button from '@/components/Button.vue';
import AddUser from '@/components/AddUser.vue';
import AddCourse from '@/components/AddCourse.vue';
import { deleteCourse as apiDeleteCourse, addUsersToCourse, removeUserFromCourse as apiRemoveUserFromCourse, fetchAllUsers, fetchCourses, fetchEditableCourses, fetchUsersForCourse, editCourse } from '@/utils/api';

// Reactive data
const showAddCourseModal = ref(false)
const showAddUserModal = ref(false)
const selectedCourse  = ref({uid: null})
const newCourse = ref({})

// Sample data - kurse
const courses = ref([
  //{
    //uid: 123,
    //name: "Course 1",
    //shortname: ['...']
  //},
  //{
    //uid: 223,
    //name: "Course 2",
    //shorname: ['...']
  //},
])

// Sample data - Users (wird normalerweise basierend auf ausgewählter Gruppe geladen)
const courseUsers = ref([
  //{
  //  selected: false,
  //  userId: 10,
  //  firstname: 'Maja',
  //  lastname: 'Biene',
  //  email: 'm.b@mail.de',
  //  title: 'Studiengangsleiter'
  //}
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

const selectCourse = async (course)=>{
    selectedCourse.value = course;
    const ru = await fetchUsersForCourse(course.uid)
    if (ru.status === 200)
        courseUsers.value = ru.body
}

const addUserToCourse = async (user) =>{
    const su = await addUsersToCourse(selectedCourse.value.uid, [user.userId])
    if (su === 200)
        courseUsers.value.push(user)
}

async function removeUserFromCourse(user) {
    const su = await apiRemoveUserFromCourse(selectedCourse.value.uid, user.userId)
    if (su === 200) {
        const index = courseUsers.value.findIndex(u => u.userId === user.userId)
        if (index > -1) {
          courseUsers.value.splice(index, 1)
        }
    }
}

async function deleteCourse() {
    const sc = await apiDeleteCourse(selectedCourse.value.uid)
    if (sc === 200) {
        const index = courses.value.findIndex(c => c.uid === selectedCourse.value.uid)
        if (index > -1) {
          courses.value.splice(index, 1)
        }
        if (courses.value.length > 0) {
            await selectCourse(courses.value[courses.value.length - 1])
        } else {
            selectedCourse.value.uid = null
        }
    }
}

async function appendCourse(course) {
    courses.value.push(course)
    showAddCourseModal.value = false
    await selectCourse(course)
}

async function saveMetadata() {
    const rc = await editCourse(selectedCourse.value.uid, selectedCourse.value)
    if (rc.status === 200)
        console.log("gespeichert.")
}

onMounted(async() => {
    const rc = await fetchEditableCourses()
    if (rc.status === 401)
        window.location.href = "/login"
    if (rc.status === 200)
        courses.value = rc.body
    const ru = await fetchAllUsers()
    if (ru.status === 200)
        allUsers.value = ru.body

    if (courses.value.length > 0)
        await selectCourse(courses.value[0])
})

</script>

<template>
    <div class="h-full">
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
        <div class="flex flex-col justify-between grow-1 gap-6 px-4" v-if="selectedCourse.uid !== null">
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
                    v-model="selectedCourse.shortname"
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
                @delete-user="removeUserFromCourse"
                />

            </div>
            <div class="flex justify-end gap-6">
                <Button
                type="secondary"
                @click="deleteCourse"
                >
                    Kurs Löschen
                </Button>
                <Button @click="saveMetadata">
                    speichern
                </Button>
            </div>
        </div>
        <!--collect all popups here-->
        <Popup title="User hinzufügen" :is-open="showAddUserModal" @close="showAddUserModal = false">
        <AddUser
        v-model="allUsers"
        @add-user="(userId)=>addUserToCourse(userId)"
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
            @create-course="(course) => appendCourse(course)"
            />
        </Popup>
    </div>
  </div>
</template>
