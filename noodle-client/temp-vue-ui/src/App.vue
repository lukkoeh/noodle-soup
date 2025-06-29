<script setup>
import Header from './components/Header.vue';
</script>

<template>
  <main class="flex justify-stretch flex-col h-full">
    <Header></Header>
    <div class="grow px-8 py-6">
      <RouterView />
    </div>
  </main>
</template>
