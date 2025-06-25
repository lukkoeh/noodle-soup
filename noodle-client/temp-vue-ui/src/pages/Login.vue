<script setup>
    import LineInput from '@/components/LineInput.vue';
    import { ref } from 'vue';
    import { login } from '@/utils/api.js'
    import Button from '@/components/Button.vue'

    const email = ref("");
    const password = ref("");

    async function loginSubmit() {
        if (await login(email.value, password.value) === 201)
            window.location.href = '/'
    }
</script>

<template>
    <div class="flex items-center justify-center min-h-full">
        <div class="login-widget flex-col flex gap-2 bg-widget shadow-xl p-4 rounded-lg">
            <LineInput
                placeholder="Email"
                v-model="email"
            />
            <LineInput
                placeholder="Passwort"
                v-model="password"
                intype="password"
            />
            <Button class="bg-accent rounded-2xl" @click="loginSubmit">Login</Button>
        </div>
        
    </div>
</template>
