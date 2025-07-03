<script setup>
import { ref } from 'vue'
import Icon from './Icon.vue';
import LineInput from './LineInput.vue';
import Button from './Button.vue';
import Popup from './Popup.vue';
import EditModal from './EditModal.vue';
import { RouterLink } from 'vue-router';
import ProfileModal from './ProfileModal.vue';

const searchQuery = ref('')
const showEditModal = ref(false)
const showProfileModal = ref(false)

</script>

<template>
  <header class="bg-main flex justify-between items-center px-4 py-2 shadow-sm w-full">
    <RouterLink to="/">
      <div>
        <!-- Logo Section -->
        <div class="bg-gray-300 px-6 py-2 rounded">
          <span class="text-gray-700 font-medium text-sm">LOGO</span>
        </div>
      </div>
    </RouterLink>
    
    <!-- Search and Icons Section -->
    <div class="flex items-center gap-2">
      <!-- Search Input -->
      <LineInput v-model="searchQuery" placeholder="Suchen" intype="search" />

      <!-- pencil Icon -->
      <Button 
      type="simple"
      @click="showEditModal = true"
      >
        <Icon
        icon="fa-pencil"
        icon-style="fa-solid"
        />
      </Button>

      <!-- Calendar Icon -->
      <Button 
      type="simple">
        <Icon
        icon="fa-calendar"
        icon-style="fa-regular"
        />
      </Button>

      <!-- User Profile Icon -->
      <Button 
      type="simple"
      @click="showProfileModal = true">
        <Icon
        icon="fa-user"
        icon-style="fa-regular"
        />
      </Button>
    </div>
    <Popup
    title="Elemente bearbeiten"
    :is-open="showEditModal"
    @close="showEditModal = false"
    >
      <EditModal
      @selected="showEditModal = false"/>
    </Popup>
    <Popup
    :is-open="showProfileModal"
    @close="showProfileModal = false"
    title="Dein Profil">
      <ProfileModal/>
    </Popup>
  </header>
</template>