<template>
  <html class="bg-white">
    <div class="flex flex-col min-h-screen">
      <main class="flex-grow">
        <router-view />
      </main>
      <FooterView />
    </div>
  </html>
</template>


<script>
import FooterView from './views/FooterView.vue'
export default {
  components: {
    FooterView
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  /* text-align: center; */
  color: #2c3e50;
  margin-top: 60px;
}
</style>
