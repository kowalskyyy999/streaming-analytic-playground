<template>
  <div class="relative isolate px-6 pt-14 lg:px-8 h-full">
    <div class="text-center">
        <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">Playground Streaming Analytics</h1>
        <div>
            <button type="submit" class="text-3xl rounded-md bg-gray-600 px-3.5 py-2.5 sm:text-2xl hover:bg-gray-900 font-semibold text-white shadow-sm"  @click="goToAuth">Try It</button>
        </div>
    </div>
  </div>
</template>

<script>
export default {
    methods: {
        goToAuth() {
            this.$router.push( {name: 'signin'})
        }
    }
}
</script>