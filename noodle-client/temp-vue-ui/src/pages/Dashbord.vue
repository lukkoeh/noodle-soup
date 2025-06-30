<script setup>
    import CourseWidget from "@/components/CourseWidget.vue";
    import Contact from "@/components/Contact.vue";
    import Overview from "@/components/Overview.vue";
    import {ref, onMounted} from "vue"
    import { fetchSelf, fetchCourses, fetchGroupsForCourse, fetchLecturersForCourse } from "@/utils/api";

    const userInfo = ref({
        firstname: 'Max'
    })
    const courseData = ref([
    {
        uid: 123,
        groups: ['ON22', 'ON22B'],
        lecturers: ['Prof. Dr. Arnulf Mester'],
        name: "T7 | Web Engineering und ganz viel Weiteres",
        bookmarked: false,
    },
    {
        uid: 124,
        groups: ['ON23', 'ON23A'],
        lecturers: ['Dr. Maria Schmidt'],
        name: "T8 | Künstliche Intelligenz und maschinelles Lernen",
        bookmarked: true,
    },
    {
        uid: 125,
        groups: ['ON24', 'ON24B'],
        lecturers: ['Prof. Dr. Peter Müller'],
        name: "T9 | Data Science für Anfänger und Fortgeschrittene",
        bookmarked: false,
    },
    {
        uid: 126,
        groups: ['ON25', 'ON25A'],
        lecturers: ['Prof. Dr. Jens Weber'],
        name: "T10 | Einführung in die Blockchain-Technologie",
        bookmarked: true,
    },
    {
        uid: 127,
        groups: ['ON26', 'ON26B'],
        lecturers: ['Dr. Laura Fuchs'],
        name: "T11 | Webentwicklung mit React und Redux",
        bookmarked: false,
    },
    {
        uid: 128,
        groups: ['ON27', 'ON27A'],
        lecturers: ['Prof. Dr. Thomas Richter'],
        name: "T12 | Datenvisualisierung mit Python und D3.js",
        bookmarked: true,
    },
    {
        uid: 129,
        groups: ['ON28', 'ON28B'],
        lecturers: ['Dr. Sabine König'],
        name: "T13 | Einführung in die Cloud-Computing-Technologien",
        bookmarked: false,
    }
]
    )

    const contacts = ref([
        {
            userId: 6,
            firstname: "Karl",
            lastname: "Lama",
            email: "karl@hats.com",
        }
    ]);

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
