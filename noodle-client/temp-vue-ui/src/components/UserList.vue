<script setup>
import { ref, computed, defineEmits } from 'vue'
import ToggleInput from './ToggleInput.vue'

// Props
const props = defineProps({
  users: {
    type: Array,
    default: () => []
  }
})

// Emits
const emit = defineEmits(['selection-change', 'edit-user', 'delete-user'])

// Reactive data
const sortField = ref(null)
const sortDirection = ref('asc')

// Computed properties
const sortedUsers = computed(() => {
  if (!sortField.value) return props.users;
  
  return [...props.users].sort((a, b) => {
    let aVal = a[sortField.value];
    let bVal = b[sortField.value];
    
    if (typeof aVal === 'string') {
      aVal = aVal.toLowerCase();
      bVal = bVal.toLowerCase();
    }
    
    if (sortDirection.value === 'asc') {
      return aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
    } else {
      return aVal > bVal ? -1 : aVal < bVal ? 1 : 0;
    }
  });
})

const isAllSelected = computed(() => {
  return props.users.length > 0 && props.users.every(user => user.selected);
})

const selectedUsers = computed(() => {
  return props.users.filter(user => user.selected);
})

// Methods
const sortBy = (field) => {
  if (sortField.value === field) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortField.value = field;
    sortDirection.value = 'asc';
  }
}

const getSortIconClass = (field) => {
  if (sortField.value !== field) return 'text-gray-400';
  return sortDirection.value === 'asc' ? 'text-blue-600 rotate-0' : 'text-blue-600 rotate-180';
}

const toggleAllSelection = (event) => {
  const isChecked = event.target.checked;
  props.users.forEach(user => {
    user.selected = isChecked;
  });
  onSelectionChange();
}

const onSelectionChange = () => {
  emit('selection-change', {
    selectedUsers: selectedUsers.value,
    allSelected: isAllSelected.value,
    selectedCount: selectedUsers.value.length
  });
}

const editUser = (user) => {
  emit('edit-user', user);
}

const deleteUser = (user) => {
  emit('delete-user', user);
}
</script>

<template>
  <div class="w-full bg-white border border-gray-200 rounded-lg overflow-hidden">
    <!-- Header -->
    <div class="bg-gray-50 border-b border-gray-200">
      <div class="grid grid-cols-12 gap-4 px-4 py-3">
        <div class="col-span-1 flex items-center">
          <ToggleInput
          v-model="isAllSelected"
          @toggled="toggleAllSelection"
          />
        </div>
        <div class="col-span-2 flex items-center space-x-1">
          <span class="text-sm font-medium text-gray-700">Vorname</span>
          <button 
            @click="sortBy('Vorname')"
            class="flex items-center justify-center w-4 h-4 hover:bg-gray-200 rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-gray-500" 
              :class="getSortIconClass('Vorname')"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
            </svg>
          </button>
        </div>
        <div class="col-span-2 flex items-center space-x-1">
          <span class="text-sm font-medium text-gray-700">Nachname</span>
          <button 
            @click="sortBy('Nachname')"
            class="flex items-center justify-center w-4 h-4 hover:bg-gray-200 rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-gray-500" 
              :class="getSortIconClass('Nachname')"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
            </svg>
          </button>
        </div>
        <div class="col-span-3 flex items-center space-x-1">
          <span class="text-sm font-medium text-gray-700">E-Mail Adresse</span>
          <button 
            @click="sortBy('eMail')"
            class="flex items-center justify-center w-4 h-4 hover:bg-gray-200 rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-gray-500" 
              :class="getSortIconClass('eMail')"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
            </svg>
          </button>
        </div>
        <div class="col-span-3 flex items-center space-x-1">
          <span class="text-sm font-medium text-gray-700">Position</span>
          <button 
            @click="sortBy('Position')"
            class="flex items-center justify-center w-4 h-4 hover:bg-gray-200 rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-gray-500" 
              :class="getSortIconClass('Position')"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
            </svg>
          </button>
        </div>
        <div class="col-span-1"></div>
      </div>
    </div>

    <!-- Body -->
    <div class="divide-y divide-gray-100">
      <div 
        v-for="(user, index) in sortedUsers" 
        :key="user.userId"
        class="grid grid-cols-12 gap-4 px-4 py-3 hover:bg-gray-50 transition-colors"
      >
        <div class="col-span-1 flex items-center">
          <ToggleInput
          v-model="user.selected"
          @toggled="onSelectionChange"
          />
        </div>
        <div class="col-span-2 flex items-center">
          <span class="text-sm text-gray-900">{{ user.Vorname }}</span>
        </div>
        <div class="col-span-2 flex items-center">
          <span class="text-sm text-gray-900">{{ user.Nachname }}</span>
        </div>
        <div class="col-span-3 flex items-center">
          <span class="text-sm text-gray-600">{{ user.eMail }}</span>
        </div>
        <div class="col-span-3 flex items-center">
          <span class="text-sm text-gray-900">{{ user.Position }}</span>
        </div>
        <div class="col-span-1 flex items-center justify-end space-x-2">
          <button 
            @click="editUser(user)"
            class="p-1 text-orange-500 hover:text-orange-700 hover:bg-orange-50 rounded transition-colors"
            title="Bearbeiten"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
            </svg>
          </button>
          <button 
            @click="deleteUser(user)"
            class="p-1 text-red-500 hover:text-red-700 hover:bg-red-50 rounded transition-colors"
            title="Löschen"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="users.length === 0" class="py-12 text-center">
      <div class="text-gray-500">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
        </svg>
        <p class="mt-2 text-sm">Keine Benutzer vorhanden</p>
      </div>
    </div>
  </div>
</template>

<style scoped>

/* Sort icon rotation animation */
.rotate-180 {
  transform: rotate(180deg);
  transition: transform 0.2s ease-in-out;
}

.rotate-0 {
  transform: rotate(0deg);
  transition: transform 0.2s ease-in-out;
}
</style>