<template>
	<h1>Convert Here</h1>
	<input type="file" @change="handleImgChange" />

	<br> <br>
	<div class="settings">
		<label for="colourCheck">Colour: </label>
		<input type="checkbox" id="colourCheck" v-model="isColour">
		<label for="fontSize">Zoom: </label>
		<input type="range" id="fontSize" v-model.number="fontSize" min="2" max="30" step="1"/>
		<span>{{ fontSize }}px</span>
	</div> 
	<br>

	<button type="submit" @click="submitImgToConverter">Submit to Preview</button>

	<pre :style="{ fontSize: `${fontSize}px` }" class="asciiArt" v-html="asciiArtHtml"></pre>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

const uploadedImg = ref<File|null>(null);
const isColour = ref(false);
const fontSize = ref(10);
const asciiArtHtml = ref<string>("");

const handleImgChange = (event: Event) => {
	const files = (event.target as HTMLInputElement).files;
	if (files && files.length > 0) {
		uploadedImg.value = files[0];
	}
};

const submitImgToConverter = async () => {
	if (!uploadedImg.value) {
		alert("Please first upload an image.");
	}

	const formData = new FormData();
	formData.append('image', uploadedImg.value);
	formData.append('config', JSON.stringify({ is_color: isColour.value }))
	
	//const response = await fetch('https://192.168.68.59:8444/convert-image', {
	const response = await fetch('https://192.168.1.95:8444/convert-image', {
		method: 'POST',
		body: formData,
	});

	const asciiGrid = await response.json();
	console.log(asciiGrid);

	asciiArtHtml.value = asciiGrid.map((row: any[]) => 
		row.map(pixel => {
			if (isColour.value && pixel.rgb) {
				const [r, g, b] = pixel.rgb;
				return `<span style="color: rgb(${r},${g},${b})">${pixel.ch}</span>`;
			} else {
				return pixel.ch;
			}
		}).join('')
	).join('<br>');
}
</script>

<style>
button {
	color: black;
}
</style>