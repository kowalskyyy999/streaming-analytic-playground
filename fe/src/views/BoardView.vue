<template>
    <div class="flex flex-row text-center bg-gray-200">
        <div class="basis-1/2">
            <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-4xl">Guess Number</h1>
            <p class="sm:text-4xl tracking-tight text-opacity-10">{{  guessNumber }}</p> 
        </div>
        <div class="basis-1/2">
            <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-4xl">Click</h1>
            <p class="sm:text-4xl tracking-tight text-opacity-10">{{  countClick }}</p> 
        </div>
        <div class="basis-1/5">
            <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">User Active</h1>
            <p class="sm:text-3xl tracking-tight text-green-500">{{ userActive }}</p>
            <!-- <div> -->
            <!--   <span class="text-xl tracking-tight text-gray-900 sm:text-2xl">Last update: </span> -->
            <!--   <span class="sm:text-xl tracking-tight text-red-500">{{ lastUpdate }}</span> -->
            <!-- </div> -->
            <!-- <p class="sm:text-xl tracking-tight text-green-500">Last Updated: {{ lastUpdate }}</p> -->
        </div>
    </div>
    <div class="bg-gray-100 t-6 grid grid-cols-1 gap-x-6 gap-y-10 sm:grid-cols-2 lg:grid-cols-4 xl:gap-x-8 content-center text-center">
        <div v-for="card in cards" :key="card.id" @click="viewCard(card)" class="group relative" >
            <div class="aspect-h-9 aspect-w-9 w-full overflow-hidden rounded-md bg-gray-200 lg:aspect-none group-hover:bg-black group-hover:opacity-50"> 
                <img :src="card.image" :alt="card.name" class="h-full w-full object-cover object-center lg:h-full lg:w-full"> 
                <p class="sm:text-2xl"> {{ card.description }}</p>
            </div>
        </div>
    </div>
    <div class="flex justify-center gap-x-10">
        <div>
            <button type="submit" class=" text-3xl rounded-md bg-green-800 px-3.5 py-2.5 sm:text-2xl hover:bg-green-900 font-semibold text-white shadow-sm"  @click="goToSubmit">Submit</button>
        </div>
        <div>
            <button type="submit" class=" text-3xl rounded-md bg-red-800 px-3.5 py-2.5 sm:text-2xl hover:bg-red-900 font-semibold text-white shadow-sm"  @click="goToReset">Reset</button>
        </div>
    </div>
    <!-- <a href="https://www.flaticon.com/free-icons/congratulation" title="congratulation icons">Congratulation icons created by Freepik - Flaticon</a> -->
</template>

<script>
import axios from 'axios'

export default {
    props: ['userId'],
    data() {
        return {
            cards: [
                { id: 1, name: 'Airpods', description: '+2', image: '../images/airpods.jpg', value: 2},
                { id: 2, name: 'Apple Watch', description: '+3', image: '../images/watch.jpg', value: 3 },
                { id: 3, name: 'Iphone', description: '+5', image: '../images/handphone.jpg', value: 5 },
                { id: 4, name: 'Macbook', description: '+7', image: '../images/laptop.jpg', value: 7 }
            ],
            guessNumber: null,
            countClick: 0,
            userActive: 0,
            lastUpdate: null,
            websocket: null
        }
    },
    methods: {
        async viewCard(card) {
            try {
                await axios.post('http://localhost:9090/api/playground/boards', {
                    userId: this.userId,
                    value: card.value
                })

                this.countClick += 1
                localStorage.setItem('countClick', this.countClick)

            } catch(error) {
                console.log('Error')
            }
        },
        async goToSubmit() {
            if (this.countClick > 0) {
                try {
                    const response = await axios.post('http://localhost:9090/api/playground/boards/submit', {
                        userId: this.userId,
                        target: this.guessNumber,
                        clicks: this.countClick
                    })
                    
                    this.countClick = 0
                    localStorage.setItem('countClick', this.countClick)

                    if (response.data.refresh == 1) {
                        window.location.reload()
                    }
                } catch(error) {
                    console.log('Error')
                }
            }
        },
        async goToReset() {
            if (this.countClick > 0) {
                try {
                    await axios.post('http://localhost:9090/api/playground/boards/reset', {
                        userId: this.userId
                    })

                    this.countClick = 0
                    localStorage.setItem('countClick', this.countClick)
                } catch(error) {
                    console.log('Error')
                }
            }
        },
        generateRandomNumber() {
            this.guessNumber = Math.floor(Math.random() * 100) + 2
        },
        handlePageRefresh(event) {
            const confirmationMessage = 'Are you sure you want to leave? Changes you made may not be saved.';
            (event || window.event).returnValue = confirmationMessage; // Gecko + IE
            return confirmationMessage; // Webkit, Safari, Chrome
        },
        loadClickCount() {
            const countClick = localStorage.getItem('countClick');
            if (countClick !== null) {
                this.countClick = Number(countClick)
            }
        },
        connectWebSocket() {
            this.websocket = new WebSocket('ws://localhost:8000/ws');
            
            this.websocket.onopen = () => {
              console.log('WebSocket connection opened');
            };

            this.websocket.onmessage = (event) => {
                const message = JSON.parse(event.data);
                console.log(message);
                this.userActive = message.user_active;
                this.lastUpdate = message.latest_ts;
            };

            this.websocket.onerror = (error) => {
                console.error('WebSocket error:', error);
            };

            this.websocket.onclose = () => {
                console.log('WebSocket connection closed');
            };
        },

    },
    created() {
        window.addEventListener('beforeunload', this.handlePageRefresh);
        this.loadClickCount();
        this.connectWebSocket();
    },
    beforeUnmount() {
        window.removeEventListener('beforeunload', this.handlePageRefresh);
        if (this.websocket) {
            this.websocket.close();
        }
    },
    mounted() {
        this.generateRandomNumber()
    }
}
</script>
