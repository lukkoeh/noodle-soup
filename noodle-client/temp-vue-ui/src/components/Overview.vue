<script setup>
import { ref } from 'vue';
import LineInput from './LineInput.vue';
import Button from './Button.vue';

const props = defineProps({
    title: {
        type: String,
        default: "Overview"
    },
    placeholder: {
        type: String,
        default: "Suchen..."
    }
});

const searchQuery = ref('');
const emit = defineEmits(['search']);

const handleSearch = () => {
    emit('search', searchQuery.value);
};

const handleScroll = (event) => {
    // Only handle wheel events with vertical movement
    if (Math.abs(event.deltaY) > Math.abs(event.deltaX)) {
        event.preventDefault();
        
        const container = event.currentTarget;
        const scrollAmount = event.deltaY * 1.5
        
        if (props.smoothScroll) {
            // Smooth scroll implementation
            container.scrollBy({
                left: scrollAmount,
                behavior: 'smooth'
            });
        } else {
            // Direct scroll for better performance with large amounts of content
            container.scrollLeft += scrollAmount;
        }
    }
};
</script>

<template>
    <div class="w-full h flex flex-col bg-white px-2">
        <!-- Header with Title and Search -->
        <div class="flex-shrink-0 flex justify-between">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-3xl font-bold text-gray-900">{{ title }}</h1>
            </div>
            <!-- search and show all button-->
            <div class="flex gap-2 items-center">
                <LineInput
                :placeholder="placeholder"
                v-model="searchQuery"
                intype="search"
                @changed="handleSearch"
                />
                <Button
                @click="handleShowAll"
                type="secondary"
                >Alles Anzeigen</Button>
            </div>

        </div>

        <!-- Horizontally Scrollable Content Area -->
        <div class="flex-1 overflow-hidden">
            <div @wheel="(event) => handleScroll(event)" class="h-full overflow-x-scroll overflow-y-hidden py-3">
                <div class="flex justify-items-start items-center gap-10 h-full min-w-max">
                    <slot />
                </div>
            </div>
        </div>
    </div>
</template>