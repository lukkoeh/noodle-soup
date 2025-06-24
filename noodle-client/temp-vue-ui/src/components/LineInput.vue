<script setup>
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
        icon: {
            type: String,
            default: '',
        }
    }
)
const type = props.password ? "password" : "text"
const model = defineModel();

const emit = defineEmits(['update:modelValue', 'changed'])

function handleInput(event) {
    const value = event.target.value;
    emit('update:modelValue', value);
    emit('changed', value);
}


</script>

<template>
    <span class="input-line bg-input text-light rounded-2xl inline-block px-2 focus-within:outline-2 ">
        <input 
        :type="type"
        :placeholder="placeholder"
        :value="model"
        @input="handleInput"
        class="rounded-2xl outline-0" />
        <input v-if="icon != ''" type="button" class="bg-accent" value="Hello"/>
    </span>
</template>
