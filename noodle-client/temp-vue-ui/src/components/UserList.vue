<script setup>
import { ref, computed, defineEmits } from 'vue'
import ToggleInput from './ToggleInput.vue'
import Icon from './Icon.vue'

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
  if (sortField.value !== field) return 'text-norm';
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
  <div class="w-full bg-main border border-gray-200 rounded-lg overflow-hidden">
    <!-- Header -->
    <div class="bg-main border-b border-gray-200">
      <div class="grid grid-cols-12 gap-4 px-4 py-3">
        <div class="col-span-1 flex items-center">
          <ToggleInput
          v-model="isAllSelected"
          @toggled="toggleAllSelection"
          />
        </div>
        <div class="col-span-2 flex items-center space-x-1">
          <span class="text-sm font-medium text-norm">Vorname</span>
          <button 
            @click="sortBy('firstname')"
            class="flex items-center justify-center w-4 h-4 hover:bg-(--c-misc-1) rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-norm" 
              :class="getSortIconClass('lastname')"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
            </svg>
          </button>
        </div>
        <div class="col-span-2 flex items-center space-x-1">
          <span class="text-sm font-medium text-norm">Nachname</span>
          <button 
            @click="sortBy('Nachname')"
            class="flex items-center justify-center w-4 h-4 hover:bg-(--c-misc-1) rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-norm" 
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
          <span class="text-sm font-medium text-norm">E-Mail Adresse</span>
          <button 
            @click="sortBy('eMail')"
            class="flex items-center justify-center w-4 h-4 hover:bg-(--c-misc-1) rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-norm" 
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
          <span class="text-sm font-medium text-norm">Position</span>
          <button 
            @click="sortBy('title')"
            class="flex items-center justify-center w-4 h-4 hover:bg-(--c-misc-1) rounded transition-colors"
          >
            <svg 
              class="w-3 h-3 text-norm" 
              :class="getSortIconClass('title')"
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
        class="grid grid-cols-12 gap-4 px-4 py-3 hover:bg-(--input) transition-colors"
      >
        <div class="col-span-1 flex items-center">
          <ToggleInput
          v-model="user.selected"
          @toggled="onSelectionChange"
          />
        </div>
        <div class="col-span-2 flex items-center">
          <span class="text-sm text-norm">{{ user.firstname }}</span>
        </div>
        <div class="col-span-2 flex items-center">
          <span class="text-sm text-norm">{{ user.lastname }}</span>
        </div>
        <div class="col-span-3 flex items-center">
          <span class="text-sm text-norm">{{ user.email }}</span>
        </div>
        <div class="col-span-3 flex items-center">
          <span class="text-sm text-norm">{{ user.title}}</span>
        </div>
        <div class="col-span-1 flex items-center justify-end space-x-2">
          <button 
            @click="editUser(user)"
            class="p-1 text-orange-500 hover:bg-(--c-misc-1) rounded transition-colors"
            title="Bearbeiten"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
            </svg>
          </button>
          <button 
            @click="deleteUser(user)"
            class="p-1 text-red-500 hover:bg-(--c-misc-1) rounded transition-colors"
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
      <div class="text-norm">
        <Icon type="users" class="text-5xl pb-4"/>
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
