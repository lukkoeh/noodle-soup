<script setup>
import ElemMarkdown from './ElemMarkdown.vue';

const props = defineProps({
    element:{
        type: Object,
        required: true,
    },
    editMode:{
        type:Boolean,
        required:true,
        default:false
    },
})
const emit = defineEmits(['update'])


</script>

<template>
    <ElemMarkdown
    v-if="props.element.type == 'markdown'"
    v-model="props.element.content"
    :edit-mode="props.editMode"
    @update="emit('update')"
    />
    <slot></slot>
</template>

<script>
    export const availableElements = [
  { type: 'markdown', label: 'Markdown', icon: 'markdown'}
]
</script>