<script setup>
    import CourseWidget from "@/components/CourseWidget.vue";
    import Contact from "@/components/Contact.vue";
    import Overview from "@/components/Overview.vue";
    import {ref, onMounted} from "vue"
    import { fetchSelf, fetchCourses, fetchGroupsForCourse, fetchLecturersForCourse } from "@/utils/api";

    const userInfo = ref({
        firstname: 'Max'
    })
    const courseData = ref([])

    const contacts = ref([]);

onMounted(async () => {
    //TODO: merge these into one request eventually, this is terrible.
    const ru = await fetchSelf()
    if (ru.status === 401)
        window.location.href = "/login"
    if (ru.status === 200)
        userInfo.value = ru.body
    const rc = await fetchCourses()
    let cs = []
    if (rc.status === 200) {
        cs = rc.body
        for (let c of cs) {
            c.lecturers = []
            const rl = await fetchLecturersForCourse(c.uid)
            if (rl.status === 200) {
                c.lecturers = rl.body
            }

            c.groups = [] //TODO
            const rg = await fetchGroupsForCourse(c.uid)
            if (rg.status === 200) {
                c.groups= rg.body
            }

            c.bookmarked = false
        }
    }
    courseData.value = cs
})
</script>

<template>
    <div
        class="flex-col flex justify-between h-full bg-main"
    >
        <h1>Hallo, {{ userInfo.firstname }}!</h1>
        <Overview
            title="Deine Kurse"
        >
            <CourseWidget
                v-for="course in courseData"
                :course="course"
            />
        </Overview>
        <Overview
            title="Deine Kontakte"
        >
            <Contact
                v-for="user in contacts"
                :user="user"
            />
        </Overview>
    </div>
</template>
