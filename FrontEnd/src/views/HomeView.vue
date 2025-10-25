<script setup lang="ts">
import MyNavbar from '@/components/MyNavbar.vue';
import MyListRow from '@/components/MyListRow.vue';
import { ref } from 'vue';

const nikName = ref("Alvaro");
const lastMsg = ref("Ciao come stai?");
const image = ref("../../public/images/image_profile_test_1.jpg");

</script>

<template>
  <MyNavbar />
  <div class=" flex flex-col pt-20 items-center h-screen backGroundMain">

    <ul class="list bg-base-100 rounded-box shadow-md">

      <li class="p-4 pb-2 text-xs opacity-60 tracking-wide">Your Chats</li>

      <MyListRow class=" hover:cursor-pointer hover:border active:opacity-50" :nik="nikName" :img="image" :last-msg="lastMsg"/>

    </ul>

  </div>
</template>

<style>





</style>
