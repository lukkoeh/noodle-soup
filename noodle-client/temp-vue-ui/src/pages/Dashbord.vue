<script setup>
    import HeaderNavigation from "@/components/Header.vue";
    import Popup from "@/components/Popup.vue"
    import CourseWidget from "@/components/CourseWidget.vue";
    import {ref} from "vue"

    const showPopup = ref(true)
</script>

<template>
    <HeaderNavigation/>
    <Popup :isOpen="showPopup" @close="showPopup = false"/>
    <h1>
        Dashboard
    </h1>
    <CourseWidget/>
</template>

<script>
</script>