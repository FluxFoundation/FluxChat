<template>


  <div class="flex flex-col h-screen justify-center items-center backGroundMain">


    <div class=" flex flex-col items-center w-120 h-140 rounded-xl bg-white">
      <form v-if="isLogin" class="flex flex-col items-center justify-center h-screen" @submit.prevent="login">
        <h2 class="mb-8 text-3xl font-bold text-black">Login</h2>
        <div v-if="apiError" role="alert"
          class="alert alert-error mb-5 flex items-center bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span class="font-bold">{{ apiError }}</span>
        </div>
        <div>
          <label for="" class="text-xs font-bold text-black">Username:</label>
          <label class="input validator bg-white text-black border-gray-200">
            <svg class="h-[1em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
                <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                <circle cx="12" cy="7" r="4"></circle>
              </g>
            </svg>
            <input v-model="user" type="text" class=" placeholder:opacity-50" required placeholder="Type your username"
              pattern="[A-Za-z][A-Za-z0-9\-]*" minlength="3" maxlength="30" title="Only letters, numbers or dash" />
          </label>
          <p class="validator-hint hidden">
            Must be 3 to 30 characters
            <br />containing only letters, numbers or dash
          </p>
        </div>
        <div>
          <label class="text-xs font-bold text-black">Password:</label>
          <label class="input validator bg-white text-black border-gray-200">
            <svg class="h-[1em] opacity-50 text-black" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
                <path
                  d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z">
                </path>
                <circle cx="16.5" cy="7.5" r=".5" fill="currentColor"></circle>
              </g>
            </svg>
            <input v-model="pw" type="password" required placeholder="Password" minlength="8"
              pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}"
              title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
              class=" placeholder:opacity-50" />

          </label>
          <p class="validator-hint hidden">
            Must be more than 8 characters, including
            <br />At least one number <br />At least one lowercase letter <br />At least one uppercase letter
          </p>
        </div>
        <button type="submit"
          class="rounded-xl mt-7 buttonColor text-white font-bold pl-8 pr-8 pt-1 pb-1 hover:cursor-pointer">Login</button>
        <p class="text-black pt-3 text-xs">If you haven't an account <a
            class="text-orange-500 hover:text-purple-600 hover:underline hover:cursor-pointer"
            @click="changeLogin">click here</a></p>

        </form>
      <form v-if="!isLogin" class="flex flex-col items-center justify-center h-screen" @submit.prevent="handleSignup">

        <h2 class="mb-8 text-3xl font-bold text-black">Sign up</h2>
        <div v-if="pwDif" role="alert" class="alert alert-error mb-5">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span class="font-bold">Error! Your password are differents</span>
        </div>
        <div v-if="apiError" role="alert"
          class="alert alert-error mb-5 flex items-center bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span class="font-bold">{{ apiError }}</span>
        </div>
        <div>


          <div>
            <label for="" class="text-xs font-bold text-black">Username:</label>
            <label class="input validator bg-white text-black border-gray-200">
              <svg class="h-[1em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
                  <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                  <circle cx="12" cy="7" r="4"></circle>
                </g>
              </svg>
              <input v-model="user" type="text" class=" placeholder:opacity-50" required
                placeholder="Type your username" pattern="[A-Za-z][A-Za-z0-9\-]*" minlength="3" maxlength="30"
                title="Only letters, numbers or dash" />
            </label>
            <p class="validator-hint hidden">
              Must be 3 to 30 characters
              <br />containing only letters, numbers or dash
            </p>
          </div>
          <div>
            <label class="text-xs font-bold text-black">Password:</label>
            <label class="input validator bg-white text-black border-gray-200">
              <svg class="h-[1em] opacity-50 text-black" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
                  <path
                    d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z">
                  </path>
                  <circle cx="16.5" cy="7.5" r=".5" fill="currentColor"></circle>
                </g>
              </svg>
              <input v-model="pw" type="password" required placeholder="Password" minlength="8"
                pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}"
                title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
                class=" placeholder:opacity-50" />

            </label>
            <p class="validator-hint hidden">
              Must be more than 8 characters, including
              <br />At least one number <br />At least one lowercase letter <br />At least one uppercase letter
            </p>
          </div>
          <div>
            <label class="text-xs font-bold text-black">confirm password:</label>
            <label class="input validator bg-white text-black border-gray-200">
              <svg class="h-[1em] opacity-50 text-black" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
                  <path
                    d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z">
                  </path>
                  <circle cx="16.5" cy="7.5" r=".5" fill="currentColor"></circle>
                </g>
              </svg>
              <input v-model="confPw" type="password" required placeholder="Password" minlength="8"
                pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}"
                title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
                class=" placeholder:opacity-50" />

            </label>
            <p class="validator-hint hidden">
              Must be more than 8 characters, including
              <br />At least one number <br />At least one lowercase letter <br />At least one uppercase letter
            </p>
          </div>
        </div>
        <button type="submit"
          class="rounded-xl mt-7 buttonColor text-white font-bold pl-8 pr-8 pt-1 pb-1 hover:cursor-pointer">Sign
          up</button>
        <p class="text-black pt-3 text-xs">If you have an account <a
            class="text-orange-500 hover:text-purple-600 hover:underline hover:cursor-pointer"
            @click="changeLogin">click here</a></p>
        </form>
    </div>


  </div>




</template>

<script setup lang="ts">
import { ref } from 'vue';

const isLogin = ref(true);
const pwDif = ref(false);
const pw = ref('');
const confPw = ref('');
const user = ref('');
const apiError = ref('');


const usernamePattern = /^[A-Za-z][A-Za-z0-9-]{2,29}$/;
const passwordPattern = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).{8,}$/;


async function handleSignup() {
  apiError.value = '';
  pwCheck();

  if (!user.value || !pw.value || !confPw.value) {
    return;
  }

  if (!usernamePattern.test(user.value)) {
    return;
  }

  if (!passwordPattern.test(pw.value)) {
    return;
  }

  if (pw.value !== confPw.value) {
    return;
  }
  const obj = { "username": user.value, "password": pw.value };
  const json = JSON.stringify(obj);

  try {
    const response = await fetch('http://127.0.0.1:8000/api/v1/signup', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: json,
    });
    const data = await response.json();

    if (!response.ok || data.message != "signup successfull" ) {
      apiError.value = data.message || 'Sign up failed';
      return;
    }


    if (data.redirectUrl) {
      window.location.href = data.redirectUrl;
      return;
    }
  } catch (err) {
    console.error(err);
    apiError.value = 'Network error. Please try again.';
  }
}

async function login() {
  apiError.value = '';
  if (!user.value || !pw.value) {
    return;
  }

  if (!usernamePattern.test(user.value)) {
    return;
  }

  if (!passwordPattern.test(pw.value)) {
    return;
  }

  const obj = { "username": user.value, "password": pw.value };
  const json = JSON.stringify(obj);

  try {
    const response = await fetch('http://127.0.0.1:8000/api/v1/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: json,
      redirect: 'follow'
    });
    const data = await response.json();

    if (!response.ok || data.message != "login successfull") {
      apiError.value = data.message || 'Login failed';
      return;
    }


    if (data.redirectUrl) {
      window.location.href = data.redirectUrl;
      return;
    }
  } catch (err) {
    console.error(err);
    apiError.value = 'Network error. Please try again.';
  }
}

function changeLogin() {

  if (isLogin.value) {

    isLogin.value = false;
  }
  else {

    isLogin.value = true;
  }
  return;
}

function pwCheck() {

  if (pw.value !== confPw.value) {

    pwDif.value = true;
  }
  else {
    pwDif.value = false;
  }

  return;
}

</script>

<style scoped>

.buttonColor {

  background-image: linear-gradient(to right, orangered, red, purple);
}

.buttonColor:active {

  background: purple;
}
</style>
