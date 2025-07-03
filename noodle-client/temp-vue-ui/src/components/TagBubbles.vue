<script setup>
import { ref } from 'vue';
defineProps({
    tagList:{
        type: Array,
        required: true,
    }
})

</script>

<template>
    <div class="flex gap-2 py-2">
        <span v-for="tag in tagList" class="px-1 py-0 bg-accent-t text-accent-l text-xs font-bold uppercase tracking-wide rounded-xl">
            {{tag}}
        </span>
    </div>
</template>