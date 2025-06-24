<script setup>
import { ref, computed } from 'vue'
import Button from './Button.vue'
import Icon from './Icon.vue'
import LineInput from './LineInput.vue'

const users = defineModel()

// Reactive data
const searchQuery = ref('')

// Define emits if needed for parent communication
const emit = defineEmits(['addUser'])

// Computed properties
const filteredUsers = computed(() => {
  if (!searchQuery.value) {
    return users.value
  }

  const query = searchQuery.value.toLowerCase()
  return users.value.filter(user =>
    user.vorname.toLowerCase().includes(query) ||
    user.nachname.toLowerCase().includes(query)
  )
})

// Methods
const handleAddUser = (userId) => {
  emit('addUser', userId);
  // Hier würde die Logik für das Erstellen implementiert werden
}



</script>

<template>
  <div>
    <LineInput
    placeholder="Benutzer Suchen"
    intype="search"
    v-model="searchQuery"
    />
    <div class="overflow-y-auto h-[50vh]">
      <div v-for="user in users" class="flex justify-between gap-2">
        <p>{{ user.Vorname + " " + user.Nachname }}</p>
        <Button type="simple" @click="() => handleAddUser(user.userId)">
          <Icon icon="fa-plus" icon-style="fa-solid"/>
        </Button>
      </div>
    </div>
  </div>
</template>
