<script setup>
import LineInput from './LineInput.vue';
import ToggleInput from './ToggleInput.vue';
import Button from './Button.vue';
import { addUsersToCourse, createCourse as apiCreateCourse } from '@/utils/api';

const newCourse = defineModel({})
const allUsers = defineModel('users')

const emit = defineEmits(['createCourse'])

const createCourse = async () => {
    const rc = await apiCreateCourse(newCourse.value.name, newCourse.value.shortname)

    if (rc.status === 200) {
        let usersToAdd = []
        for (const u of allUsers.value) {
            if (u.selected) usersToAdd.push(u.userId)
        }
        if (usersToAdd.length > 0) {
            const ru = await addUsersToCourse(rc.body.uid, usersToAdd)
            if (ru === 200)
                emit('createCourse', rc.body)
                return
        }
        emit('createCourse', rc.body)
    }
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
            v-model="newCourse.shortname"
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
