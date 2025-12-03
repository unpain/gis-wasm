<script setup lang="ts">
import { onMounted } from 'vue'
import testGeoJSON from './test.json'
import __wbg_init, { get_feature_length } from '@/pkg/gis_wasm'
onMounted(async () => {
  await __wbg_init()
  console.log(get_feature_length(JSON.stringify(testGeoJSON)))
})
</script>
<template></template>
<style scoped></style>
