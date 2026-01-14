<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue'
onMounted(() => { })
const counter = ref(0)
</script>

<template>
    <p>Counter: {{ counter }}</p>
</template>

<style scoped>

</style>