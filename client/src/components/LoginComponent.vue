<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <div class="card bg-white w-full max-w-md rounded-xl shadow-xl">
      <div class="card-body p-8">
        <div class="text-3xl font-semibold text-center text-gray-800 mb-6">
          Log In
        </div>

        <Form
          :validation-schema="loginValidationSchema"
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

          <button
            class="btn btn-primary w-full py-3 text-lg rounded-lg hover:bg-indigo-600 transition-all duration-200"
            :disabled="isLoggingIn"
          >
            <div class="flex items-center justify-center gap-2">
              <span>Log In</span>
              <span>
                <ProgressSpinner
                  v-if="isLoggingIn"
                  style="width: 20px; height: 20px; stroke: #4a00ff"
                  strokeWidth="4"
                  animationDuration="1s"
                />
              </span>
            </div>
          </button>
        </Form>

        <div class="text-center mt-4 text-sm text-gray-600">
          <p>
            Not registered yet?
            <RouterLink
              :to="routes.SIGNUP"
              class="text-blue-500 hover:underline"
              >Sign Up</RouterLink
            >
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Form, Field, ErrorMessage } from 'vee-validate'
import UserService from '@/shared/services/user.service'
import ProgressSpinner from 'primevue/progressspinner'
import { useNotification } from '@kyvg/vue3-notification'
import { useMutation } from '@tanstack/vue-query'
import { STORAGE_KEYS } from '@/shared/keys'
import { loginValidationSchema } from '@/shared/validation-schemas/login.schema'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user.store'
import { routes } from '../router/routes'

const router = useRouter()

const { notify } = useNotification()

const userStore = useUserStore()

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
      text: error?.response?.data?.message || 'Something went wrong!',
    })
  },
})

const onSubmit = async (values) => {
  loginUser(values)
}
</script>
