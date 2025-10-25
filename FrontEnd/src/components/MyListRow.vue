<template>
  <li class="list-row pr-20">
        <div><img class="size-10 rounded-full" :src="img" /></div>
        <div>
          <div>{{nik}}</div>
          <div class="text-xs font-semibold opacity-60">{{ lastMsg }}</div>
        </div>
      </li>
</template>

<script setup lang="ts">


  const props = defineProps({

    nik: String,
    img: String,
    lastMsg: String
  });
</script>
