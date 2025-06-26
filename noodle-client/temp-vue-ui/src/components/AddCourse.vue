<script setup>
import LineInput from './LineInput.vue';
import ToggleInput from './ToggleInput.vue';
import Button from './Button.vue';

const newCourse = defineModel({})
const allUsers = defineModel('users')

const emit = defineEmits(['createCourse'])

const createCourse = () => {
    emit('createCourse')
}

</script>

<template>
    <div class="flex flex-col gap-4">
        <div class="flex justify-between gap-4">
        <div>
            <p>Titel</p>
            <LineInput
            placeholder="Titel"
            v-model="newCourse.name"
            ></LineInput>
        </div>
            <div>
            <p>Kürzel</p>
            <LineInput
            placeholder="Kürzel"
            v-model="newCourse.short"
            ></LineInput>
            </div>
        </div>
        <div
        class="flex flex-col gap-2">
            <div
            class="flex gap-4 justify-between font-bold">
                <p>Auswahl</p>
                <p>Vorname</p>
                <p>Nachname</p>
            </div>
            <div
            class="overflow-y-auto h-max"
            >
                <div
                class="flex gap-4 justify-between"
                v-for="user in allUsers"
                >
                    <ToggleInput
                    v-model="user.selected"/>
                    <p>{{ user.firstname }}</p>
                    <p>{{ user.lastname }}</p>
                </div>
            </div>
        </div>
        <Button
        @click="createCourse"
        >erstellen</Button>
    </div>
</template>