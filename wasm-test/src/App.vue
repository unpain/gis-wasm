<script setup lang="ts">
import { onMounted } from 'vue'
import __wbg_init, { get_bbox } from '@/pkg/gis_wasm'
import wzlgGeoJson from '@/assets/test.json'
onMounted(async () => {
  await __wbg_init()
})
const calc = () => {
  const time = performance.now()
  get_bbox(JSON.stringify(wzlgGeoJson))
  console.log(`Calculation took ${performance.now() - time} ms`)
}
</script>
<template>
  <button @click="calc">计算bbox</button>
</template>
<style scoped></style>
