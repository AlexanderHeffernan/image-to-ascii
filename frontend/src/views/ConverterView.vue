<template>
	<h1>Convert Here</h1>
	<input style="color: white" type="file" @change="handleImgChange" />

	<br> <br>
	<div class="settings">
		<label for="colourCheck">Colour: </label>
		<input type="checkbox" id="colourCheck" v-model="isColour">
		<label for="brightness">Brightness: </label>
		<input type="range" id="brightness" v-model.number="brightness" min="0.1" max="3" step="0.1">
		<span>{{ brightness }}</span>
		<label for="contrast">Contrast: </label>
		<input type="range" id="contrast" v-model.number="contrast" min="0.1" max="3" step="0.1">
		<span>{{ contrast }}</span>
		<br>
		<label for="width">Width (10-500):</label>
		<input type="number" id="width" v-model.number="width" min="10" max="500">
		<label for="height">Height (10-500) *not working:</label>
		<input type="number" id="height" v-model.number="height" min="10" max="500">
	</div> 
	<br>

	<button type="submit" @click="submitImgToConverter">Submit to Preview</button>

	<label for="fontSize">Zoom: </label>
	<input type="range" id="fontSize" v-model.number="fontSize" min="2" max="30" step="1"/>
	<span>{{ fontSize }}px</span>
	<pre :style="{ fontSize: `${fontSize}px` }" class="asciiArt" v-html="asciiArtHtml"></pre>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

const uploadedImg = ref<File|null>(null);
const isColour = ref(false);
const brightness = ref(1);
const contrast = ref(1);
const width = ref(200);
const height = ref(100);
const isLandscape = ref(true);
const aspectRatio = ref(0);
const aspectLock = ref(true);
const fontSize = ref(10);
const asciiArtHtml = ref<string>("");

const handleImgChange = (event: Event) => {
	const files = (event.target as HTMLInputElement).files;
	if (files && files.length > 0) {
		uploadedImg.value = files[0];

		// Get aspect ratio and whether it is landscape
		const img = new Image();
		const objectUrl = URL.createObjectURL(files[0]);
		img.src = objectUrl;
		img.onload = () => {
			aspectRatio.value = img.naturalWidth/img.naturalHeight;
			isLandscape.value = img.naturalWidth >= img.naturalHeight
			console.log(`Image dimensions: ${img.naturalWidth}x${img.naturalHeight}`);
			URL.revokeObjectURL(objectUrl);
		};
			img.onerror = () => {
			URL.revokeObjectURL(objectUrl);
		};
	}
};

const submitImgToConverter = async () => {
	if (!uploadedImg.value) {
		alert("Please first upload an image.");
	}

	const formData = new FormData();
	formData.append('image', uploadedImg.value);
	formData.append('config', JSON.stringify({ output_width: width.value, brightness_factor: brightness.value, contrast_factor: contrast.value, is_color: isColour.value }))
	
	const response = await fetch('https://192.168.68.59:8444/convert-image', {
	//const response = await fetch('https://192.168.1.95:8444/convert-image', {
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
input {
	color: black;
}
</style>