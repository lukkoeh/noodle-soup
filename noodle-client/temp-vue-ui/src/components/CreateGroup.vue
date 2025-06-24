<script setup>
import { ref, computed } from 'vue'
import Button from './Button.vue'
import ToggleInput from './ToggleInput.vue'
import LineInput from './LineInput.vue'

// Reactive data
const searchQuery = ref('')

const users = ref([
  {
    id: 1,
    vorname: 'Hänsel',
    nachname: 'Wald',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 2,
    vorname: 'Hänsel',
    nachname: 'Bald',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 3,
    vorname: 'Hänsel',
    nachname: 'Mald',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 4,
    vorname: 'Hänsel',
    nachname: 'Walz',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 5,
    vorname: 'Hänsel',
    nachname: 'Salz',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 6,
    vorname: 'Hänsel',
    nachname: 'Mald',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 7,
    vorname: 'Hänsel',
    nachname: 'Walz',
    email: 'h.wald@mail.de',
    selected: false
  },
  {
    id: 8,
    vorname: 'Hänsel',
    nachname: 'Salz',
    email: 'h.wald@mail.de',
    selected: false
  }
])

// Computed properties
const filteredUsers = computed(() => {
  if (!searchQuery.value) {
    return users.value
  }

  const query = searchQuery.value.toLowerCase()
  return users.value.filter(user =>
    user.vorname.toLowerCase().includes(query) ||
    user.nachname.toLowerCase().includes(query) ||
    user.email.toLowerCase().includes(query)
  )
})

// Methods
const handleCreate = () => {
  const selectedUsers = users.value.filter(user => user.selected)
  console.log('Selected users:', selectedUsers)
  // Hier würde die Logik für das Erstellen implementiert werden
}

// Define emits if needed for parent communication
const emit = defineEmits(['create', 'userSelected'])

// Example of emitting events to parent
const emitCreate = () => {
  const selectedUsers = users.value.filter(user => user.selected)
  emit('create', selectedUsers)
}

const data = ref({
  gname: '',
  bereich: '',
  kuerzel: '',
})
</script>

<template>
  <div class="flex flex-col justify-between gap-7">
    <!-- Header -->
    <div class="mb-6">
      <h2>Titel</h2>
      <LineInput v-model="data.gname" placeholder="Gruppen-Name" />
    </div>

    <!-- Bereich und Kürzel -->
    <div class="flex justify-between gap-4 mb-6">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Bereich</label>
        <LineInput placeholder="Bereich" v-model="data.bereich" />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Kürzel</label>
        <LineInput placeholder="Kürzel" v-model="data.kuerzel" />
      </div>
    </div>

    <!-- User Section -->
    <div class="flex gap-4 justify-between">
      <h2 class="text-lg font-medium text-gray-900 mb-4">User</h2>

      <!-- Search -->
      <LineInput placeholder="Suche" v-model="searchQuery" intype="search" />
    </div>

    <div class="overflow-hidden">
      <!-- User Table -->
      <div class="border border-gray-200 rounded-md overflow-scroll max-h-[30vh]">
        <!-- Table Header -->
        <div
          class="bg-gray-50 px-4 py-3 grid grid-cols-12 gap-4 text-sm font-medium text-gray-700 border-b border-gray-200">
          <div class="col-span-1"></div>
          <div class="col-span-3">Vorname</div>
          <div class="col-span-3">Nachname</div>
          <div class="col-span-5">E-Mail-Adresse</div>
        </div>

        <!-- User Rows -->
        <div v-for="user in filteredUsers" :key="user.id"
          class="px-4 py-3 grid grid-cols-12 gap-4 text-sm border-b border-gray-200 last:border-b-0 hover:bg-gray-50">
          <div class="col-span-1 flex items-center">
            <ToggleInput v-model="user.selected" />
          </div>
          <div class="col-span-3 text-gray-900">{{ user.vorname }}</div>
          <div class="col-span-3 text-gray-900">
            {{ user.nachname }}
          </div>
          <div class="col-span-5 text-gray-600">{{ user.email }}</div>
        </div>
      </div>
    </div>

    <!-- Create Button -->
    <div class="flex justify-end mt-6">
      <Button @click="handleCreate">
        erstellen
      </Button>
    </div>
  </div>
</template>
