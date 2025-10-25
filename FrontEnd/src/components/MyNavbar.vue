<template>
  <div class="navbar bg-base-100 shadow-sm">
  <div class="flex-1">
    <a class="btn btn-ghost text-xl">FluxChat</a>
  </div>
  <div class="flex gap-2">
    <input type="text" placeholder="Search" class="input input-bordered w-24 md:w-auto" />
    <div class="dropdown dropdown-end">
      <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
        <div class="w-10 rounded-full">
          <img
            alt="Tailwind CSS Navbar component"
            :src="default_img_profile" />
        </div>
      </div>
      <ul
        tabindex="-1"
        class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
        <li>
          <a class="justify-between">
            Profile
            <span class="badge text-yellow-500">Soon</span>
          </a>
        </li>
        <li><a>Settings
          <span class="badge text-yellow-500">Soon</span>
        </a></li>
        <li><router-link to="/login">Logout</router-link></li>
      </ul>
    </div>
  </div>
</div>

<router-view/>
</template>

<script setup lang="ts">

import { ref } from 'vue';
import { RouterLink, RouterView } from 'vue-router';



const default_img_profile = ref("../../public/images/profile_default.jpg");

</script>


