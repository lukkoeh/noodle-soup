<script setup>
import { ref } from 'vue';
import Popup from './Popup.vue';

const emit = defineEmits(["bookmarked"])

const props = defineProps({
    user: {
        type: Object,
        required: true,
        default: {
            userId: 69,
            firstname: "theuser",
            lastname: "lastname",
            email: "john@fourtytwo.com",
        }
    },
});
const showContact = ref(false);

</script>

<template>
    <div
    class="w-30 h-30 bg-input rounded-full shadow-lg relative flex-shrink-0 flex items-center justify-center"
    @click="showContact = true"
    >
        <p class="text-5xl font-bold">
            {{ props.user.firstname[0] }}{{ props.user.lastname[0] }}
        </p>
        <Popup
            title=""
            :is-open="showContact"
            @close="showContact = false"
        >
            <div
                class="flex flex-col justify-between items-center gap-6"
            >
                <div
                class="w-30 h-30 bg-input rounded-full shadow-lg relative flex-shrink-0 flex items-center justify-center"
                @click="showContact = true"
                >
                    <p class="text-5xl font-bold">
                        {{ props.user.firstname[0] }}{{ props.user.lastname[0] }}
                    </p>
                </div>
                
                <p
                class="font-bold text-2xl"
                >{{ props.user.firstname }} {{ props.user.lastname }}</p>
                <a :href="'mailto:' + props.user.email">{{ props.user.email }}</a>
            </div>
        </Popup>
    </div>
</template>