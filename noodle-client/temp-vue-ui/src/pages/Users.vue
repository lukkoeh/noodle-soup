<script setup>
import { ref, onMounted } from 'vue'
import { fetchAllUsers } from '@/utils/api.js'
import UserList from '@/components/UserList.vue'

const userList = ref([])

onMounted(async () => {
    await loadAllUsers()
})

function handleSelectionChange(data) {
    console.log('Ausgewählte Benutzer:', data.selectedUsers)
}
function handleEditUser(user) {
    console.log('Bearbeite Benutzer:', user)
}
function handleDeleteUser(user) {
    console.log('Lösche Benutzer:', user)
}

async function loadAllUsers() {
    const r = await fetchAllUsers()
    if (r.status === 401)
        window.location.href = "/login"

    userList.value = r.body
}

</script>

<template>
    <div>
        <UserList :users="userList" @selection-change="handleSelectionChange" @edit-user="handleEditUser"
            @delete-user="handleDeleteUser" />
    </div>
</template>
