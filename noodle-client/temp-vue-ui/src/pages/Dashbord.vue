<script setup>
    import HeaderNavigation from "@/components/Header.vue";
    import Popup from "@/components/Popup.vue"
    import CourseWidget from "@/components/CourseWidget.vue";
    import Contact from "@/components/Contact.vue";
    import Overview from "@/components/Overview.vue";
    import {ref} from "vue"

    const showPopup = ref(false)
    const userInfo = ref({
        firstname: 'Max'
    })
    const courseData = ref([
    {
        uid: 123,
        tags: ['ON22', 'ON22B'],
        dozenten: ['Prof. Dr. Arnulf Mester'],
        description: "T7 | Web Engineering und ganz viel Weiteres",
        bookmarked: false,
    },
    {
        uid: 124,
        tags: ['ON23', 'ON23A'],
        dozenten: ['Dr. Maria Schmidt'],
        description: "T8 | Künstliche Intelligenz und maschinelles Lernen",
        bookmarked: true,
    },
    {
        uid: 125,
        tags: ['ON24', 'ON24B'],
        dozenten: ['Prof. Dr. Peter Müller'],
        description: "T9 | Data Science für Anfänger und Fortgeschrittene",
        bookmarked: false,
    },
    {
        uid: 126,
        tags: ['ON25', 'ON25A'],
        dozenten: ['Prof. Dr. Jens Weber'],
        description: "T10 | Einführung in die Blockchain-Technologie",
        bookmarked: true,
    },
    {
        uid: 127,
        tags: ['ON26', 'ON26B'],
        dozenten: ['Dr. Laura Fuchs'],
        description: "T11 | Webentwicklung mit React und Redux",
        bookmarked: false,
    },
    {
        uid: 128,
        tags: ['ON27', 'ON27A'],
        dozenten: ['Prof. Dr. Thomas Richter'],
        description: "T12 | Datenvisualisierung mit Python und D3.js",
        bookmarked: true,
    },
    {
        uid: 129,
        tags: ['ON28', 'ON28B'],
        dozenten: ['Dr. Sabine König'],
        description: "T13 | Einführung in die Cloud-Computing-Technologien",
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

</script>

<template>
    <div
        class="flex-col flex justify-between h-full"
    >
        <HeaderNavigation/>
        <h1>Hallo {{ userInfo.firstname }}</h1>
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