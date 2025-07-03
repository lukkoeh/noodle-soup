<script setup>
import { ref } from 'vue';
import { RouterLink } from 'vue-router';
import Icon from './Icon.vue';

const rLinkStyles = ref("bg-input text-norm min-w-30 min-h-30 flex items-center justify-center rounded-2xl text-5xl")
const emit = defineEmits(['selected'])

</script>

<template>
    <div>
        <div class="flex gap-4 justify-center">
            <RouterLink to="/users" :class="rLinkStyles" @click="emit('selected')">
                <Icon icon="fa-user" icon-style="fa-solid" />
            </RouterLink>
            <RouterLink to="/groups" :class="rLinkStyles" @click="emit('selected')">
                <Icon icon="fa-users-line" icon-style="fa-solid" />
            </RouterLink>
            <RouterLink to="/courses" :class="rLinkStyles" @click="emit('selected')">
                <Icon icon="fa-sheet-plastic" icon-style="fa-solid" />
            </RouterLink>
        </div>
    </div>

</template>