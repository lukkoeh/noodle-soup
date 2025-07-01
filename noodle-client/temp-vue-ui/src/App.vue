<script setup>
import Header from './components/Header.vue';
import { useRoute } from 'vue-router';

const route = useRoute();
</script>

<template>
  <main class="flex justify-stretch flex-col h-full">
    <Header
    v-if="route.path !== '/login'"/>
    <div class="grow px-8 py-6">
      <RouterView />
    </div>
  </main>
</template>
