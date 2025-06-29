<script setup>
import { ref, onMounted } from 'vue'
import { fetchAllUsers } from '@/utils/api.js'
import UserList from '@/components/UserList.vue'
import Button from '@/components/Button.vue'
import Popup from '@/components/Popup.vue'
import Icon from '@/components/Icon.vue'
import LineInput from '@/components/LineInput.vue'
import CreateUserModal from '@/components/CreateUserModal.vue'

const showAddUserModal = ref(false);
const userList = ref([]);

const newUser = ref({})

onMounted(async () => {
    await loadAllUsers()
})

function handleSelectionChange(data) {
    console.log('Ausgewählte Benutzer:', data.selectedUsers)
}
function handleCreateUser(createUser){

    console.log(createUser)
    newUser.value = {}
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
    <div
    class="h-full">
        <div
        class="flex justify-between pb-4">
            <p
            class="text-2xl">
                Alle User
                <Button
                @click="showAddUserModal = true"
                >
                <Icon type="plus"/>
                </Button>
            </p>
            <LineInput
            intype="search"
            placeholder="Benutzer suchen"/>
        </div>
        <UserList :users="userList" @selection-change="handleSelectionChange" @edit-user="handleEditUser"
            @delete-user="handleDeleteUser" />

        <Popup
        title="Benutzer hinzufügen"
        :is-open="showAddUserModal"
        @close="showAddUserModal = false">
            <CreateUserModal
            v-model="newUser"
            @create-user="(userData) => handleCreateUser(userData)"
            />
        </Popup>
    </div>
</template>
