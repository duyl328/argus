<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h2>Essential Links</h2>
    <ul>
      <li><a href="https://vuejs.org" target="_blank">Core Docs</a></li>
      <li><a href="https://forum.vuejs.org" target="_blank">Forum</a></li>
      <li><a href="https://gitter.im/vuejs/vue" target="_blank">Gitter Chat</a></li>
      <li><a href="https://twitter.com/vuejs" target="_blank">Twitter</a></li>
      <br>
      <li><a href="http://vuejs-templates.github.io/webpack/" target="_blank">Docs for This Template</a></li>
    </ul>
    <h2>Ecosystem</h2>
    <ul>
      <li><a href="http://router.vuejs.org/" target="_blank">vue-router</a></li>
      <li><a href="http://vuex.vuejs.org/" target="_blank">vuex</a></li>
      <li><a href="http://vue-loader.vuejs.org/" target="_blank">vue-loader</a></li>
      <li><a href="https://github.com/vuejs/awesome-vue" target="_blank">awesome-vue</a></li>
    </ul>
    <h2>Masonry plugin demo</h2>
    <button v-on:click="reDraw">redrawVueMasonry</button>
    <div v-masonry transition-duration="3s" item-selector=".item" class="masonry-container">
      <div v-masonry-tile class="item" v-bind:key="index" v-for="(item, index) in blocks">
        <p>{{item.title}} - {{index}}</p>
      </div>
    </div>
  </div>
</template>

<script>

import Vue from 'vue'
//import {VueMasonryPlugin} from '../../node_modules/vue-masonry/src/masonry.plugin';
import {VueMasonryPlugin} from 'vue-masonry';

Vue.use(VueMasonryPlugin)

export default {
  name: 'hello',
  data () {
    return {
      msg: 'Welcome to Your Vue.js App',
      blocks: [
              {
                title: 'nice block'
              },
              {
                title: 'another shiny block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
              {
                title: 'the last block'
              },
      ]
    }
  },
  mounted: function(){
    this.$redrawVueMasonry();
  },
  methods: {
    reDraw: function(){
      this.$redrawVueMasonry();
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}
.item {
  border: 1px solid #ac0;
  width: 200px;
}
.masonry-container {
      width: 55%;
    margin: 0 auto;
}
</style>
