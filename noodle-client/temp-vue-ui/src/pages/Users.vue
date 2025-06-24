<script setup>
import { ref } from 'vue'
import UserList from '@/components/UserList.vue'

const userList = ref(
    [
        {
            selected: false,
            userId: 'abcdef123',
            Vorname: 'Miriam',
            Nachname: 'Anders',
            eMail: 'm.anders@mail.de',
            Position: 'Sachbearbeiter'
        },
        {
            selected: false,
            userId: 'ghijkl456',
            Vorname: 'Max',
            Nachname: 'Bertram',
            eMail: 'm.b@mail.de',
            Position: 'Depp vom Dienst'
        },
    ]
)
function handleSelectionChange(data) {
    console.log('Ausgewählte Benutzer:', data.selectedUsers)
}
function handleEditUser(user) {
    console.log('Bearbeite Benutzer:', user)
}
function handleDeleteUser(user) {
    console.log('Lösche Benutzer:', user)
}
</script>

<template>
    <div>
        <UserList :users="userList" @selection-change="handleSelectionChange" @edit-user="handleEditUser"
            @delete-user="handleDeleteUser" />
    </div>
</template>