<template>
    <div class="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">  
      <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
        <div v-if="alertMessage" class="text-red-600 border rounded-none border-red-600 border-dotted bg-red-100 ring-inset">{{ alertMessage }}</div>
        <form @submit.prevent="signUp" class="space-y-6">
          <div>
            <label for="email" class="block text-sm font-medium leading-6 text-gray-900">Username</label>
            <div class="mt-2">
              <input id="username" v-model="username" type="text" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 sm:text-sm sm:leading-6" placeholder="username" />
            </div>
          </div>
  
          <div>
            <div class="flex items-center justify-between">
              <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
            </div>
            <div class="mt-2">
              <input id="password" v-model="password" type="password" autocomplete="current-password" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 sm:text-sm sm:leading-6" placeholder="password"/>
            </div>
          </div>
  
          <div>
            <button type="submit" class="flex w-full justify-center rounded-md bg-gray-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-gray-900 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Sign Up</button>
          </div>
        </form>
      </div>
    </div>
  </template>

<script>
import axios from 'axios'

export default{
  data() {
    return {
      alertMessage: ''
    }
  },
  methods: {
    async signUp() {
      try {
        const response = await axios.post('http://localhost:9090/api/auth/signup', {
          username: this.username,
          password: this.password
        })

        if (response.status == 201) {
          this.$router.push({ name: 'board', params: { userId: response.data.userid } })
        }
      } catch(error) {
        this.alertMessage = error.response.data.message
      }
    }
  },
}
</script>