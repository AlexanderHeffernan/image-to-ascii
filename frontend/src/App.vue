<template>
	<div class="app-content">
		<nav>
			<h1 style="padding: 0px !important">PixCha</h1>
			<div class="router-links">
				<router-link class="router-link" to="/">Home</router-link>
				<router-link class="router-link" to="/converter">Converter</router-link>
			</div>
		</nav>
		<router-view/>
	</div>
</template>

<style>
* {
	color: white;
	margin: 0;
	padding: 0;
}

body {
	background-color: rgb(17, 17, 17);
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

.app-content {
	height: 100vh;
}

.router-link {
	padding-left: 10px;
	text-decoration: none;
}

.router-link:hover {
	text-decoration: underline;
}
</style>