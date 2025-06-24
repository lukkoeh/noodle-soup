<script setup>
import { ref } from 'vue';

const props = defineProps({
    title: {
        type: String,
        default: "Overview"
    },
    placeholder: {
        type: String,
        default: "Search..."
    }
});

const searchQuery = ref('');
const emit = defineEmits(['search']);

const handleSearch = () => {
    emit('search', searchQuery.value);
};
</script>

<template>
    <div class="w-full h flex flex-col bg-white">
        <!-- Header with Title and Search -->
        <div class="flex-shrink-0 p-6 border-b border-gray-200">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-3xl font-bold text-gray-900">{{ title }}</h1>
            </div>

            <!-- Search Bar -->
            <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                </div>
                <input v-model="searchQuery" @input="handleSearch" type="text" :placeholder="placeholder"
                    class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-lg leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500" />
            </div>
        </div>

        <!-- Horizontally Scrollable Content Area -->
        <div class="flex-1 overflow-hidden">
            <div class="h-full overflow-x-scroll overflow-y-hidden py-3">
                <div class="flex justify-items-start items-center gap-10 h-full min-w-full">
                    <slot />
                </div>
            </div>
        </div>
    </div>
</template>