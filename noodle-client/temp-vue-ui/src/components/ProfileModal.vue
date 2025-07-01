<script setup>
import Button from './Button.vue';
import { useRouter } from 'vue-router';

const router = useRouter()

const handleLogout = () => {
    //TODO add Logout logic

    router.push("/login")
}
</script>

<template>
    <div
    class="flex flex-col gap-4 justify-between items-center">
    <Button
    @click="handleLogout">Logout</Button>
    </div>
</template>