<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <div class="card bg-white w-full max-w-md rounded-xl shadow-xl">
      <div class="card-body p-8">
        <div class="text-3xl font-semibold text-center text-gray-800 mb-6">
          Sign Up
        </div>

        <Form
          :validation-schema="signupValidationSchema"
          @submit="onSubmit"
          class="space-y-6"
        >
          <div class="space-y-1">
            <label for="email" class="block text-sm font-medium text-gray-700">
              Email
            </label>
            <Field
              name="email"
              type="email"
              validateOnInput
              placeholder="Enter your email"
              class="input input-bordered w-full rounded-lg border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
            />
            <ErrorMessage name="email" class="text-red-500 text-sm" />
          </div>

          <div class="space-y-1">
            <label
              for="password"
              class="block text-sm font-medium text-gray-700"
            >
              Password
            </label>
            <Field
              name="password"
              type="password"
              validateOnInput
              placeholder="Enter your password"
              class="input input-bordered w-full rounded-lg border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
            />
            <ErrorMessage name="password" class="text-red-500 text-sm" />
          </div>

          <div class="space-y-1">
            <label
              for="confirmPassword"
              class="block text-sm font-medium text-gray-700"
            >
              Confirm Password
            </label>
            <Field
              name="confirmPassword"
              type="password"
              validateOnInput
              placeholder="Confirm your password"
              class="input input-bordered w-full rounded-lg border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
            />
            <ErrorMessage name="confirmPassword" class="text-red-500 text-sm" />
          </div>

          <button
            class="btn btn-primary w-full py-3 text-lg rounded-lg hover:bg-indigo-600 transition-all duration-200"
            :disabled="isRegistering || isLoggingIn"
          >
            <div class="flex items-center justify-center gap-2">
              <span>Sign Up</span>
              <span>
                <ProgressSpinner
                  v-if="isRegistering || isLoggingIn"
                  style="width: 20px; height: 20px; stroke: #4a00ff"
                  strokeWidth="4"
                  animationDuration="1s"
                />
              </span>
            </div>
          </button>
        </Form>

        <!-- "Already have an account?" text with link to log in -->
        <div class="text-center mt-4 text-sm text-gray-600">
          <p>
            Already have an account?
            <RouterLink :to="routes.LOGIN" class="text-blue-500 hover:underline"
              >Log In</RouterLink
            >
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Form, Field, ErrorMessage } from 'vee-validate'
import { signupValidationSchema } from '@/shared/validation-schemas/signup.schema'
import { useMutation } from '@tanstack/vue-query'
import UserService from '@/shared/services/user.service'
import ProgressSpinner from 'primevue/progressspinner'
import { useNotification } from '@kyvg/vue3-notification'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user.store'
import { STORAGE_KEYS } from '@/shared/keys'
import { routes } from '../router/routes'

const router = useRouter()

const { notify } = useNotification()

const userStore = useUserStore()

const { mutate: registerUser, isPending: isRegistering } = useMutation({
  mutationFn: async (values) => {
    return UserService.registerUser({
      email: values.email,
      password: values.password,
    })
  },
  onSuccess: async (_data, variables) => {
    loginUser({
      email: variables.email,
      password: variables.password,
    })
  },
  onError: (error) => {
    notify({
      type: 'error',
      title: 'Error',
      text: error?.response?.data?.message || 'Registration failed!',
    })
  },
})

const { mutate: loginUser, isPending: isLoggingIn } = useMutation({
  mutationFn: async (values) => {
    return UserService.loginUser({
      email: values.email,
      password: values.password,
    })
  },
  onSuccess: (response) => {
    const user = response.user
    userStore.setUser(user)

    const token = response.token
    localStorage.setItem(STORAGE_KEYS.ACCESS_TOKEN, token)

    router.push({ name: 'chats' })
  },
  onError: (error) => {
    notify({
      type: 'error',
      title: 'Error',
      text: error?.response?.data?.message || 'Login failed!',
    })
  },
})

const onSubmit = (values) => {
  registerUser(values)
}
</script>
