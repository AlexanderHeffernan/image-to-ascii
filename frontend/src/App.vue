<template>
  <nav>
    <h1>PixCha</h1>
    <div>
      <router-link class="router-link" to="/">Home</router-link>
      <router-link class="router-link" to="/converter">Converter</router-link>
    </div>
  </nav>
  <router-view/>
</template>

<style>
* {
  color: white;
  margin: 0;
  padding: 0;
}

body {
  background-color: black;
  padding-left: 100px;
  padding-right: 100px;
}

nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 30px;
  padding-bottom: 30px;
  border-bottom: 1px solid white;
}

.router-link {
  padding-left: 10px;
}

</style>