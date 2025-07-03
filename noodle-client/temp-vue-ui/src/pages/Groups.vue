<script setup>
import { ref, computed, onMounted } from 'vue';
import { fetchGroups, fetchAllUsers, fetchUsersOfGroup, addUsersToGroup, removeUsersFromGroup } from '@/utils/api.js';
import UserList from '@/components/UserList.vue';
import CreateGroup from '@/components/CreateGroup.vue';
import Popup from '@/components/Popup.vue';
import Icon from '@/components/Icon.vue';
import LineInput from '@/components/LineInput.vue';
import Button from '@/components/Button.vue';
import AddUser from '@/components/AddUser.vue';

// Reactive data
const groupSearchQuery = ref('')
const userSearchQuery = ref('')
const selectedGroup = ref(null)
const selectedUsers = ref([])
const showCreateGroupModal = ref(false)
const showAddUserModal = ref(false)

// Sample data - Groups
// TODO: turn this into an empty array
const groups = ref([
  {
    id: 1,
    kuerzel: 'ON22',
    titel: 'Onlinemedien 22',
    bereich: 'Wirtschaft'
  },
  {
    id: 2,
    kuerzel: 'ON23',
    titel: 'Onlinemedien 23',
    bereich: 'Wirtschaft'
  },
  {
    id: 3,
    kuerzel: 'ON24',
    titel: 'Onlinemedien 24',
    bereich: 'Wirtschaft'
  },
  {
    id: 4,
    kuerzel: 'ON27',
    titel: 'Onlinemedien 27',
    bereich: 'Wirtschaft'
  },
  {
    id: 5,
    kuerzel: 'BWL22',
    titel: 'BWL 22',
    bereich: 'Wirtschaft'
  },
  {
    id: 6,
    kuerzel: 'BWL23',
    titel: 'BWL 23',
    bereich: 'Wirtschaft'
  },
  {
    id: 7,
    kuerzel: 'BWL24',
    titel: 'BWL 24',
    bereich: 'Wirtschaft'
  }
])

// Sample data - Users (wird normalerweise basierend auf ausgewählter Gruppe geladen)
// TODO: turn this into an empty array
const usersInGroup = ref([
  {
    selected: false,
    userId: 'user1',
    Vorname: 'Maja',
    Nachname: 'Biene',
    email: 'm.b@mail.de',
    Position: 'Studiengangsleiter'
  },
  {
    selected: false,
    userId: 'user2',
    Vorname: 'Maja',
    Nachname: 'Biene',
    email: 'm.b@mail.de',
    Position: 'Studiengangsleiter'
  },
  {
    selected: false,
    userId: 'user3',
    Vorname: 'Maja',
    Nachname: 'Biene',
    email: 'm.b@mail.de',
    Position: 'Studiengangsleiter'
  },
  {
    selected: false,
    userId: 'user4',
    Vorname: 'Maja',
    Nachname: 'Biene',
    email: 'm.b@mail.de',
    Position: 'Studiengangsleiter'
  },
  {
    selected: false,
    userId: 'user5',
    Vorname: 'Maja',
    Nachname: 'Biene',
    email: 'm.b@mail.de',
    Position: 'Studiengangsleiter'
  },
  {
    selected: false,
    userId: 'user6',
    Vorname: 'Maja',
    Nachname: 'Biene',
    email: 'm.b@mail.de',
    Position: 'Studiengangsleiter'
  }
])

const allUsers = ref([]);

// Computed properties
const filteredGroups = computed(() => {
  if (!groupSearchQuery.value) return groups.value

  const query = groupSearchQuery.value.toLowerCase()
  return groups.value.filter(group =>
    group.name.toLowerCase().includes(query) ||
    group.shortname.toLowerCase().includes(query)
  )
})

const filteredUsers = computed(() => {
  if (!userSearchQuery.value) return usersInGroup.value

  const query = userSearchQuery.value.toLowerCase()
  return usersInGroup.value.filter(user =>
    user.firstname.toLowerCase().includes(query) ||
    user.lastname.toLowerCase().includes(query) ||
    user.email.toLowerCase().includes(query)
  )
})

// Methods
const selectGroup = async (group) => {
  selectedGroup.value = group
  const r = await fetchUsersOfGroup(group.groupId)
  if (r.status === 200)
    usersInGroup.value = r.body
}

const handleUserSelectionChange = (data) => {
  selectedUsers.value = data.selectedUsers
  console.log('User Auswahl geändert:', data)
}

const handleEditUser = (user) => {
  console.log('Bearbeite User:', user)
}

const handleDeleteUser = async (user) => {
  console.log('Lösche User:', user)
  const status = await removeUsersFromGroup(selectedGroup.value.groupId, [user.userId])
  // User aus der Liste entfernen
  if (status === 200) {
    const index = usersInGroup.value.findIndex(u => u.userId === user.userId)
    if (index > -1) {
      usersInGroup.value.splice(index, 1)
    }
  }
}
const openAddModal = () => {
  showAddUserModal.value = true;
}

const addUserToGroup = async (user) => {
  const r = await addUsersToGroup(selectedGroup.value.groupId, [user.userId])
  if (r === 201)
    usersInGroup.value.push(user)
  // Hier würde ein Modal oder eine andere UI zum Hinzufügen von Usern geöffnet
}

function appendGroupToList(group) {
  groups.value.push(group)
  showCreateGroupModal.value = false
}

const removeSelectedUsers = async() => {
  console.log('Entferne ausgewählte User:', selectedUsers.value)
  const status = await removeUsersFromGroup(selectedGroup.value.groupId, selectedUsers.value.map(u => u.userId))
  // User aus der Liste entfernen
  if (status === 200) {
    selectedUsers.value.forEach(user => {
        const index = usersInGroup.value.findIndex(u => u.userId === user.userId)
        if (index > -1) {
          usersInGroup.value.splice(index, 1)
        }
    })
    selectedUsers.value = []
  }
}

const saveChanges = () => {
  console.log('Speichere Änderungen')
  // Hier würden die Änderungen gespeichert werden
}

onMounted(async () => {
  const rg = await fetchGroups()

  if (rg.status === 200) 
    groups.value = rg.body

  const ru = await fetchAllUsers()

  if (ru.status === 200) 
    allUsers.value = ru.body
})

</script>

<template>
  <div class="flex h-screen bg-main">
    <!-- Left Sidebar -->
    <div class="w-80 bg-main border-r border-gray-200 flex flex-col">
      <!-- Header -->
      <div class="p-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-semibold text-norm">Alle Usergruppen</h2>
          <button @click="showCreateGroupModal = true"
            class="w-8 h-8 flex items-center justify-center bg-red-500 text-norm rounded-full hover:bg-red-600 transition-colors">
            <Icon icon="fa-plus" icon-style="fa-solid"></Icon>
          </button>

        </div>

        <!-- Search Groups -->
        <!-- <LineInput placeholder="Suche Usergruppe" icon="fa-magnifying-glass" icon-style="fa-solid" /> -->
        <div class="relative mt-3">
          <input v-model="groupSearchQuery" type="text" placeholder="Suche Usergruppe"
            class="w-full px-3 py-2 pr-10 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm" />
          <svg class="absolute right-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-norm" fill="none"
            stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
          </svg>
        </div>
      </div>

      <!-- Groups List -->
      <div class="flex-1 overflow-y-auto">
        <div class="p-2">
          <div v-for="group in filteredGroups" :key="group.groupId" @click="selectGroup(group)" :class="[
            'flex items-center justify-between p-3 rounded-lg cursor-pointer transition-colors mb-1',
            selectedGroup?.groupId === group.groupId
              ? 'bg-(--c-misc-1) border border-blue-200'
              : 'hover:bg-(--c-misc-1)'
          ]">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 bg-(--c-misc-1) rounded-lg flex items-center justify-center">
                <span class="text-xs font-medium text-norm">{{ group.shortname}}</span>
              </div>
              <div>
                <div class="text-sm font-medium text-norm">{{ group.name}}</div>
                <!-- <div class="text-xs text-gray-500">{{ group.name}}</div> -->
              </div>
            </div>
            <div class="text-xs text-norm">{{ group.name}}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col">
      <!-- Content Header -->
      <div v-if="selectedGroup" class="border-b border-gray-200 p-6">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-xl font-semibold text-norm">{{ selectedGroup.name}}</h1>
            <div class="flex items-center space-x-4 mt-2 text-sm text-norm">
              <!-- <div> -->
                <!-- <span class="font-medium">Bereich:</span> -->
                <!-- <span class="ml-1">{{ selectedGroup.bereich }}</span> -->
              <!-- </div> -->
              <div>
                <span class="font-medium">Kürzel: </span>
                <span class="ml-1">{{ selectedGroup.shortname }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Users Section -->
      <div v-if="selectedGroup" class="flex-1 p-6">
        <div class="bg-main rounded-lg border border-gray-200">
          <!-- Users Header -->
          <div class="p-4 border-b border-gray-200">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-medium text-norm">User</h3>
              <Button @click="openAddModal">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                </svg>
                <span>User hinzufügen</span>
              </Button>
            </div>

            <!-- Search Users -->
            <div class="relative mt-3">
              <input v-model="userSearchQuery" type="text" placeholder="Suche User"
                class="w-64 px-3 py-2 pr-10 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm" />
              <svg class="absolute right-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-norm" fill="none"
                stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
              </svg>
            </div>
          </div>

          <!-- User List Component -->
          <div class="p-4">
            <UserList :users="filteredUsers" @selection-change="handleUserSelectionChange" @edit-user="handleEditUser"
              @delete-user="handleDeleteUser" />
          </div>

          <!-- Actions -->
          <div v-if="selectedUsers.length > 0" class="p-4 border-t border-gray-200 bg-gray-50">
            <div class="flex items-center justify-between">
              <span class="text-sm text-norm">
                {{ selectedUsers.length }} User ausgewählt
              </span>
              <div class="flex space-x-3">
                <button @click="removeSelectedUsers"
                  class="px-4 py-2 text-sm font-medium text-red-600 bg-white border border-red-300 rounded-md hover:bg-red-50 transition-colors">
                  User aus Gruppe entfernen
                </button>
                <button @click="saveChanges"
                  class="px-4 py-2 text-sm font-medium text-white bg-red-500 border border-transparent rounded-md hover:bg-red-600 transition-colors">
                  Speichern
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <svg class="mx-auto h-12 w-12 text-norm" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z">
            </path>
          </svg>
          <h3 class="mt-2 text-sm font-medium text-norm">Keine Gruppe ausgewählt</h3>
          <p class="mt-1 text-sm text-norm">Wählen Sie eine Gruppe aus der Liste aus.</p>
        </div>
      </div>
    </div>
    <!-- // collect all popups here -->

    <Popup title="Neue Usergruppe" :is-open="showCreateGroupModal" @close="showCreateGroupModal = false">
      <CreateGroup v-model="allUsers" @create="(g) => appendGroupToList(g)"></CreateGroup>
    </Popup>
    <Popup title="Neuer User" :is-open="showAddUserModal" @close="showAddUserModal = false">
      <AddUser
      v-model="allUsers"
      @add-user="(data)=>addUserToGroup(data)"
      />
    </Popup>
  </div>
</template>
