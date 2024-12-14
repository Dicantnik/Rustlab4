<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <div
      class="flex flex-col space-y-6 w-full max-w-sm px-8 py-8 bg-white rounded-lg shadow-lg"
    >
      <h1 class="text-3xl font-semibold text-center text-gray-800">Welcome!</h1>

      <RouterLink :to="routes.SIGNUP">
        <button
          class="btn btn-primary w-full py-3 text-lg transform-none hover:scale-105 transition-all duration-200"
        >
          Sign Up
        </button>
      </RouterLink>

      <RouterLink :to="routes.LOGIN">
        <button
          class="btn btn-outline w-full py-3 text-lg hover:bg-gray-800 hover:text-white transition-all duration-200"
        >
          Log In
        </button>
      </RouterLink>
    </div>
  </div>
</template>

<script setup>
import { routes } from '../router/routes'
</script>
