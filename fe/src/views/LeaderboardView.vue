<template>
  <div class="flex min-h-screen bg-gray-100">
    <!-- Sidebar -->
    <div class="w-64 bg-white shadow-md">
      <div class="p-4 text-lg font-semibold border-b">
        Leaderboard
      </div>
      <ul class="mt-4">
        <li class="p-4">
          <label for="dynamic-select" class="block text-sm font-medium text-gray-700 mb-2">Select Target (2 - 100):</label>
          <select id="dynamic-select" v-model="selectedOption" @change="updateChart" class="block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500">
            <option v-for="option in options" :key="option.value" :value="option.value">
                {{ option.text }}
            </option>
          </select>
        </li> 
      </ul>
    </div>

    <!-- Main Content -->
    <div class="flex-1 p-8">
      <!-- <ChartComponent :chartData="chartData" :options="chartOptions" /> -->
      <LeaderboardComponent :selectedOption="selectedOption" />
    </div>
  </div>
</template>

<script>
// import ChartComponent from './ChartComponent.vue';
import LeaderboardComponent from './LeaderboardComponent.vue';
export default {
  data() {
    return {
      selectedOption: '2',
      options: [],
    };
  },
  created() {
    for (let i = 2; i<=100; i++) {
      this.options.push({ value: `${i}`, text: `Target ${i}`});
    }
  },
  components: {
    // ChartComponent,
    LeaderboardComponent
  },
  methods: {
    updateChart() {
      // this.$emit('update-chart', this.selectedOption);
      this.$forceUpdate();
    },
    generateDataValues() {
      console.log(this.selectedOption);
      switch (this.selectedOption) {
        case '2':
          return [10, 20, 30, 40];
        case '3':
          return [15, 25, 35, 45];
        case '4':
          return [20, 30, 40, 50];
        default:
          return [5, 15, 25, 35];
      }
    },
  },
  computed: {
    chartData() {
      const dataValues = this.generateDataValues();
      return {
        labels: ['Label 1', 'Label 2', 'Label 3', 'Label 4'],
        datasets: [
          {
            label: `Data for ${this.selectedOption}`,
            backgroundColor: '#42A5F5',
            data: dataValues
          }
        ]
      };
    },
    chartOptions() {
      return {
        responsive: true,
        maintainAspectRatio: false
      };
    }
  },
};
</script>

<style>
/* Tailwind CSS handles most of the styling */
</style>
