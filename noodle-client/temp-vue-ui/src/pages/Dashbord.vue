<script setup>
    import HeaderNavigation from "@/components/Header.vue";
    import Popup from "@/components/Popup.vue"
    import CourseWidget from "@/components/CourseWidget.vue";
    import Overview from "@/components/Overview.vue";
    import {ref} from "vue"

    const showPopup = ref(false)
    const userInfo = ref({
        name: 'Max'
    })
    const courseData = ref([
    {
        tags: ['ON22', 'ON22B'],
        dozenten: ['Prof. Dr. Arnulf Mester'],
        description: "T7 | Web Engineering und ganz viel Weiteres",
        bookmarked: false,
    },
    {
        tags: ['ON23', 'ON23A'],
        dozenten: ['Dr. Maria Schmidt'],
        description: "T8 | Künstliche Intelligenz und maschinelles Lernen",
        bookmarked: true,
    },
    {
        tags: ['ON24', 'ON24B'],
        dozenten: ['Prof. Dr. Peter Müller'],
        description: "T9 | Data Science für Anfänger und Fortgeschrittene",
        bookmarked: false,
    },
    {
        tags: ['ON25', 'ON25A'],
        dozenten: ['Prof. Dr. Jens Weber'],
        description: "T10 | Einführung in die Blockchain-Technologie",
        bookmarked: true,
    },
    {
        tags: ['ON26', 'ON26B'],
        dozenten: ['Dr. Laura Fuchs'],
        description: "T11 | Webentwicklung mit React und Redux",
        bookmarked: false,
    },
    {
        tags: ['ON27', 'ON27A'],
        dozenten: ['Prof. Dr. Thomas Richter'],
        description: "T12 | Datenvisualisierung mit Python und D3.js",
        bookmarked: true,
    },
    {
        tags: ['ON28', 'ON28B'],
        dozenten: ['Dr. Sabine König'],
        description: "T13 | Einführung in die Cloud-Computing-Technologien",
        bookmarked: false,
    }
]
)

</script>

<template>
    <HeaderNavigation/>
    <Popup :isOpen="showPopup" @close="showPopup = false"/>
    <h1>Hallo {{ userInfo.name }}</h1>
    <Overview
    title="Deine Kurse"
    >
        <CourseWidget
            v-for="item in courseData"
            :course="item"
        />
    </Overview>
    <Overview
    title="Deine Kontakte"
    >
        <CourseWidget
            v-for="item in courseData"
            :course="item"
        />
    </Overview>
</template>

<script>
</script>