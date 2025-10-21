<template>


  <div class="flex flex-col h-screen justify-center items-center backGroundMain">
    <div class=" flex flex-col items-center p-25 rounded-xl bg-white">
      <h2 class="mb-8 text-3xl font-bold text-black">Login</h2>
      <div>
        <label for="" class="text-xs font-bold text-black">Username:</label>
        <label class="input validator bg-white text-black border-gray-200">
          <svg class="h-[1em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
              <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </g>
          </svg>
          <input type="text" class=" placeholder:opacity-50" required placeholder="Type your username"
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
          <input type="password" required placeholder="Password" minlength="8"
            pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}"
            title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
            class=" placeholder:opacity-50" />

        </label>
        <p class="validator-hint hidden">
          Must be more than 8 characters, including
          <br />At least one number <br />At least one lowercase letter <br />At least one uppercase letter
        </p>
      </div>
      <button class="rounded-xl backGroundMain mt-5 text-white font-bold pl-4 pr-4 pt-1 pb-1">Login</button>
    </div>
  </div>


  <!--<br>
  <br>
  <br>
  <div>
    <h2>Sign Up</h2>
    <div>
      <label for="username">Username:</label>
      <input class="border" type="text" name="username" id="username" required>
    </div>
    <div>
      <label for="pw">Password:</label>
      <input class="border" type="password" name="pw" id="pw" required>
    </div>
    <div>
      <label for="pw">Confirm password:</label>
      <input class="border" type="password" name="pw" id="pw" required>
    </div>
    <button class="border">Sign In</button>
  </div>
  -->
</template>

<script setup lang="ts">

</script>

<style scoped>
.backGroundMain {

  background-image: linear-gradient(305deg, orangered, purple);
}
</style>
