<script setup>
    import LineInput from '@/components/LineInput.vue';
    import { ref } from 'vue';
    import { login } from '@/utils/api.js'

    const email = ref("");
    const password = ref("");

    function login_submit(){
        if (login(email.value, password.value))
            window.location.href = '/'
    }
</script>

<template>
    <div class="flex items-center justify-center min-h-full">
        <div class="login-widget flex-col flex gap-2 bg-widget shadow-xl p-4 rounded-lg">
            <LineInput
                placeholder="Email"
                @changed="(value) => email=value"
            />
            <LineInput
                placeholder="Passwort"
                :password=true
                @changed="(value)=>password=value"
            />
            <input class="bg-accent rounded-2xl" type="button" @click="login_submit" value="Login"/>
        </div>
        
    </div>
</template>
