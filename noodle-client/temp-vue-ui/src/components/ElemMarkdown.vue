<script setup>
import { computed, ref } from 'vue'
import { marked } from 'marked'

const props = defineProps({
    editMode:{
        type:Boolean,
        default:true,
    },
})

const content = defineModel();
const emit = defineEmits(['update'])

// Configure marked options
marked.setOptions({
  breaks: true,        // Support line breaks
  gfm: true,          // GitHub Flavored Markdown
  headerIds: false,   // Don't generate header IDs
  mangle: false       // Don't mangle email addresses
})

const renderedMarkdown = computed(() => {
  if (!content.value) return ''
  
  try {
    return marked.parse(content.value)
  } catch (error) {
    console.error('Markdown parsing error:', error)
    return '<p>Error parsing markdown</p>'
  }
})

const handleInput = (event) => {
    content.value = event.target.value;
    emit('update');
}
</script>

<template>
    <div
    v-if="props.editMode == true"
    class="flex justify-stretch items-stretch">
        <textarea
        class="w-80"
        @input="(event)=>{handleInput(event)}"
        >{{ content }}</textarea>
        <div class="markdown-wrapper" v-html="renderedMarkdown"></div>
    </div>
    <div
    v-if="props.editMode == false"
    class="markdown-wrapper break-all w-max-full" v-html="renderedMarkdown"></div>
</template>