<script setup>
import { computed, ref } from 'vue';
import Button from './Button.vue';
import Icon from './Icon.vue';

const props = defineProps(
    {
        placeholder: {
            type: String,
            default: ''
        },
        password: {
            type: Boolean,
            default: false,
        },
        intype: {
            type:String,
            default: 'normal',
            validator: value => ['normal', 'search', 'add', 'password'].includes(value)
        },
    }
)
const visible = ref(!(props.type == 'password'));
const icon = ref('');
const iconStyle = ref('');
const type = computed(() => visible.value ? "text" : "password");
const model = defineModel();

switch(props.intype){
    case 'search':
    icon.value = 'fa-magnifying-glass';
    iconStyle.value = 'fa-solid';
    break;
    case 'add':
    icon.value = 'fa-plus';
    iconStyle.value = 'fa-solid';
    break;
    case 'password':
    icon.value = 'fa-eye';
    iconStyle.value = 'fa-regular';
    break;
}


const emit = defineEmits(['update:modelValue', 'changed', 'clicked'])

function handleInput(event) {
    const value = event.target.value;
    emit('update:modelValue', value);
    emit('changed', value);
}
function handleClick(event){
    visible.value = props.intype == 'password' ? !visible.value : true;
    emit('clicked');
}


</script>

<template>
    <span class="input-line bg-input text-light rounded-2xl inline-block px-2 focus-within:outline-2 ">
        <input :type="type" :placeholder="placeholder" :value="model" @input="handleInput"
            class="rounded-2xl outline-0" />
        <Button
            v-if="icon"
            type="simple"
            @click="handleClick"
            >
            <Icon :icon="icon" :icon-style="iconStyle"></Icon>
        </Button>
    </span>
</template>
