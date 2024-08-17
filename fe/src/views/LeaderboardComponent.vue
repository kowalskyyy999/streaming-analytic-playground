<template>
  <div class="p-6 bg-gray-100 min-h-screen">
    <div class="space-y-8">
      <div class="p-6 bg-white shadow-md rounded-lg">
        <h2 class="text-2xl font-semibold mb-4">Fastest</h2>
          <table class="min-w-full divide-y divide-gray-200">
            <thead>
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">No</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Username</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Time (s)</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Rank</th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr v-for="(user, index) in paginatedLeaderboardFastest" :key="user.id">
                <td class="px-6 py-4 whitespace-nowrap">{{ index + 1 + (currentPageFastest -1) * itemsPerPageFastest }}</td>
                <td class="px-6 py-4 whitespace-nowrap">{{ user.username }}</td>
                <td class="px-6 py-4 whitespace-nowrap">{{ user.times }}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <i :class="getRankIcon(user.ranking)" class="text-2xl"></i>
                </td>
              </tr>
            </tbody>
          </table>
        <div class="flex justify-between items-center mt-4">
          <button @click="prevPageFastest" :disabled="currentPage === 1" class="px-4 py-2 bg-gray-200 rounded-md text-gray-600 hover:bg-gray-300 disabled:bg-gray-100 disabled:text-gray-400">Previous</button>
          <span class="text-gray-600">Page {{ currentPageFastest }} of {{ totalPages }}</span>
          <button @click="nextPageFastest" :disabled="currentPage === totalPages" class="px-4 py-2 bg-gray-200 rounded-md text-gray-600 hover:bg-gray-300 disabled:bg-gray-100 disabled:text-gray-400">Next</button>
        </div>
      </div>
      <div class="p-6 bg-white shadow-md rounded-lg">
        <h2 class="text-2xl font-semibold mb-4">Shortest</h2>
          <table class="min-w-full divide-y divide-gray-200">
            <thead>
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">No</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Username</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Click</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Rank</th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr v-for="(user, index) in paginatedLeaderboardShortest" :key="user.id">
                <td class="px-6 py-4 whitespace-nowrap">{{ index + 1 + (currentPageShortest -1) * itemsPerPageShortest }}</td>
                <td class="px-6 py-4 whitespace-nowrap">{{ user.username }}</td>
                <td class="px-6 py-4 whitespace-nowrap">{{ user.clicks }}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <i :class="getRankIcon(user.ranking)" class="text-2xl"></i>
                </td>
              </tr>
            </tbody>
          </table>
        <div class="flex justify-between items-center mt-4">
          <button @click="prevPageShortest" :disabled="currentPageShortest === 1" class="px-4 py-2 bg-gray-200 rounded-md text-gray-600 hover:bg-gray-300 disabled:bg-gray-100 disabled:text-gray-400">Previous</button>
          <span class="text-gray-600">Page {{ currentPageShortest }} of {{ totalPages }}</span>
          <button @click="nextPageShortest" :disabled="currentPageShortest === totalPages" class="px-4 py-2 bg-gray-200 rounded-md text-gray-600 hover:bg-gray-300 disabled:bg-gray-100 disabled:text-gray-400">Next</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'
export default {
  name: 'LeaderboardComponent',
  props: ['selectedOption'],
  data() {
    return {
      leaderboardFastest: [],
      leaderboardShortest: [],
      currentPageFastest: 1,
      itemsPerPageFastest: 5,
      currentPageShortest: 1,
      itemsPerPageShortest: 5,
    };
  },
  computed: {
    totalPages() {
      return Math.ceil(this.leaderboardFastest.length / this.itemsPerPageFastest);
    },
    paginatedLeaderboardFastest() {
      const start = (this.currentPageFastest - 1) * this.itemsPerPageFastest;
      const end = start + this.itemsPerPageFastest;
      return this.leaderboardFastest.slice(start, end);
    },
    paginatedLeaderboardShortest() {
      const start = (this.currentPageShortest - 1) * this.itemsPerPageShortest;
      const end = start + this.itemsPerPageShortest;
      return this.leaderboardShortest.slice(start, end);
    }
  },
  methods: {
    getRankIcon(rank) {
      switch (rank) {
        case 1:
          return 'fas fa-trophy text-yellow-500'; // Gold cup
        case 2:
          return 'fas fa-trophy text-gray-400'; // Silver cup
        case 3:
          return 'fas fa-trophy text-orange-600'; // Bronze cup
        default:
          return 'fas fa-trophy text-black'; // Black cup
      }
    },
    async fetchLeaderBoardDataFastest() {
      const urlFastest = `${process.env.VUE_APP_ANALYTIC_HOST}/api/v1/fastest/${this.selectedOption}`;
      await axios.get(urlFastest)
        .then(response => {
          this.leaderboardFastest = response.data || [];
        })
        .catch(error => {
          console.error('Error fetching data:', error);
        });
    },
    async fetchLeaderBoardDataShortest() {
      const url = `${process.env.VUE_APP_ANALYTIC_HOST}/api/v1/shortest/${this.selectedOption}`;
      await axios.get(url)
        .then(response => {
          this.leaderboardShortest = response.data || [];
        })
        .catch(error => {
          console.error('Error fetching data:', error);
        });
    },
    nextPageFastest() {
      if (this.currentPageFastest < this.totalPages) {
        this.currentPageFastest++;
      }
    },
    prevPageFastest() {
      if (this.currentPageFastest > 1) {
        this.currentPageFastest--;
      }
    },
    nextPageShortest() {
      if (this.currentPageShortest < this.totalPages) {
        this.currentPageShortest++;
      }
    },
    prevPageShortest() {
      if (this.currentPageShortest > 1) {
        this.currentPageShortest--;
      }
    },
  },
  watch: {
      selectedOption(){
        this.fetchLeaderBoardDataShortest();
        this.fetchLeaderBoardDataFastest();
    }
  },
  mounted() {
    this.fetchLeaderBoardDataFastest();
    this.fetchLeaderBoardDataShortest();
  }
};
</script>

<style scoped>
/* Tailwind CSS is used for styling, no custom CSS needed */
</style>
