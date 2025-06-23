<script setup>
    import LineInput from '@/components/LineInput.vue';
    import { ref } from 'vue';
    const password = ref("");
    const username = ref("");

    function login_submit(){
        console.log("Username: ", username);
        console.log("Passwort: ", password);
    }

</script>

<template>
    <div class="flex items-center justify-center min-h-full">
        <div class="login-widget flex-col flex gap-2 bg-widget shadow-xl p-4 rounded-lg">
            <LineInput
                placeholder="Benutzername"
            />
            <LineInput
                placeholder="Passwort"
                password="true"
                @changed="(value)=>password=value"
            />
            <input class="bg-accent rounded-2xl" type="button" @click="login_submit" value="Login"/>
        </div>
        
    </div>
</template>