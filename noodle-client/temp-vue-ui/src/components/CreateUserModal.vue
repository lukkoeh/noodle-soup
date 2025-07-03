<script setup>
import LineInput from './LineInput.vue';
import Button from './Button.vue';

const user = defineModel()
const emit = defineEmits(['createUser'])

</script>
<template>
    <div
    class="flex flex-col gap-4 justify-between items-stretch">
        <div
        class="flex justify-between items-stretch">
            <label class="flex flex-col gap-2">
                Vorname*
                <LineInput
                placeholder="Vorname"
                v-model="user.firstname"/>
            </label>
            <label class="flex flex-col gap-2">
                Nachname*
                <LineInput
                placeholder="Nachname"
                v-model="user.lastname"/>
            </label>
        </div>
        <label class="flex flex-col gap-2">
            E-Mail*
            <LineInput
            placeholder="E-Mail"
            v-model="user.email"/>
        </label>
        <label class="flex flex-col gap-2">
            Position
            <LineInput
            placeholder="Position"
            v-model="user.title"/>
        </label>
        <div class="flex justify-between">
            <label>Usergruppen</label>
            <label>Berechtigungen</label>
        </div>
        <div class="flex justify-between">
            <p>Der von Dir erstellte User erhält ein automatisch generiertes Passwort an die angegebene E-Mail Adresse.</p>
            <Button
            @click="emit('createUser', user)"
            >erstellen</Button>
        </div>
    </div>
</template>