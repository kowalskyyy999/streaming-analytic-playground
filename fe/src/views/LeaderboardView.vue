<template>
  <div class="flex h-screen bg-gray-200">
    <div class="bg-gray-800 text-white w-64 py-4 px-6">
      <h1 class="text-2xl font-bold mb-8">Leaderboard</h1>
        <nav>
          <div>
            <button type="button" id="menu-button" aria-expanded="true" aria-haspopup="true">Test</button>
          </div>
          <div role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
            <div class="py-1" role="none">
      <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
              <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="menu-item-0">Account settings</a>
              <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="menu-item-1">Support</a>
              <a href="#" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="menu-item-2">License</a>
              <form method="POST" action="#" role="none">
                <button type="submit" class="block w-full px-4 py-2 text-left text-sm text-gray-700" role="menuitem" tabindex="-1" id="menu-item-3">Sign out</button>
      </form>
    </div>
  </div>
        </nav>
      <!-- <nav>
        <div>
        <button @click="toggleAccordion" class="w-full text-left bg-gray-700 text-white py-2 px-4">
          Target
        </button>
        <div v-show="isAccordionOpen" class="bg-gray-800 text-white p-4">
          <form @submit.prevent="submitTarget">
            <label for="target" class="block text-gray-300 mb-2">Select Target:</label>
            <select v-model="target" id="target" class="w-full p-2 mb-4 text-black">
              <option v-for="n in 99" :key="n" :value="n + 1">{{ n + 1 }}</option>
            </select>
            <button type="submit" class="bg-blue-500 text-white py-2 px-4 rounded">Set Target</button>
          </form>
        </div>
      </div>
      </nav> -->
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      isAccordionOpen: false,
      target: 2,
    };
  },
  methods: {
    toggleAccordion() {
      this.isAccordionOpen = !this.isAccordionOpen;
    },
    submitTarget() {
      alert(`Target set to: ${this.target}`);
      // Handle the target value (e.g., send it to a backend or store it in a state management solution)
    },
  },
};
</script>

<style scoped>
</style>


<!--  -->